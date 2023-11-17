module Main exposing (main)

import Browser
import Element exposing (Element, text)
import Element.Input as Input
import Html exposing (Html)
import SearchBox
import Sort
import Sort.Dict as Dict exposing (Dict)
import Sort.Set as Set exposing (Set)
import Task as Task_
import Time exposing (Posix)


type TaskId
    = TaskId Int


type UserId
    = UserId Int


type TagId
    = TagId Int


type TaskSize
    = Small
    | Medium
    | Large


sizeToString : TaskSize -> String
sizeToString size =
    case size of
        Small ->
            "small"

        Medium ->
            "medium"

        Large ->
            "large"



-- type TaskBundle = TaskBundle (List TaskBlueprint)


type Mode
    = ViewingBoard
    | AddingTask
    | AddingUser


type TaskStatus
    = ToDo
    | InProgress
    | Done


type alias User =
    { name : String
    }


type alias Task =
    { title : String
    , description : String
    , size : TaskSize
    , assignees : List UserId
    , created : Posix
    , updated : Posix
    , due : Maybe Posix
    , tags : List TagId
    }


type alias TaskView =
    { title : String
    , description : String
    , size : TaskSize
    , created : Posix
    , updated : Posix
    , due : Maybe Posix
    , tags : List TagId
    , assignees : List String
    , blocks : List String
    , blockedBy : List String
    , expanded : Bool
    , taskId : TaskId
    }


type alias NewTask r =
    { r
        | newTaskTitle : String
        , newTaskDescription : String
        , newTaskAssignees : List UserId
        , newTaskSize : TaskSize
        , newTaskBlocks : List TaskId
        , newTaskBlockedBy : List TaskId
        , newTaskDue : Maybe Posix
        , newTaskTags : List TagId
        , newTaskStatus : TaskStatus
        , newTaskUserSearchBox : SearchBox.State
        , newTaskUserSearchBoxText : String
        , newTaskBlocksSearchBox : SearchBox.State
        , newTaskBlocksSearchBoxText : String
        , newTaskBlockedBySearchBox : SearchBox.State
        , newTaskBlockedBySearchBoxText : String
    }


type alias Model =
    { mode : Mode
    , users : Dict UserId User

    -- Tasks
    , tasks : Dict TaskId Task
    , blocks : Dict TaskId (Set TaskId)
    , blockedBy : Dict TaskId (Set TaskId)
    , expandedTasks : Set TaskId

    -- Board
    , toDo : List TaskId
    , inProgress : List TaskId
    , done : List TaskId

    -- New Task
    , newTaskTitle : String
    , newTaskDescription : String
    , newTaskAssignees : List UserId
    , newTaskSize : TaskSize
    , newTaskBlocks : List TaskId
    , newTaskBlockedBy : List TaskId
    , newTaskDue : Maybe Posix
    , newTaskTags : List TagId
    , newTaskStatus : TaskStatus
    , newTaskUserSearchBox : SearchBox.State
    , newTaskUserSearchBoxText : String
    , newTaskBlocksSearchBox : SearchBox.State
    , newTaskBlocksSearchBoxText : String
    , newTaskBlockedBySearchBox : SearchBox.State
    , newTaskBlockedBySearchBoxText : String

    -- New User
    , newUserName : String
    }


taskView : TaskId -> Model -> Maybe TaskView
taskView taskId model =
    let
        taskQuery =
            Dict.get taskId model.tasks

        assignees =
            case taskQuery of
                Nothing ->
                    []

                Just task ->
                    List.foldl
                        (\userId acc ->
                            case Dict.get userId model.users of
                                Nothing ->
                                    acc

                                Just user ->
                                    user.name :: acc
                        )
                        []
                        task.assignees

        blocks =
            case Dict.get taskId model.blocks of
                Nothing ->
                    []

                Just taskIds ->
                    Set.foldl
                        (\blockedTaskId acc ->
                            case Dict.get blockedTaskId model.tasks of
                                Nothing ->
                                    acc

                                Just task ->
                                    task.title :: acc
                        )
                        []
                        taskIds

        blockedBy =
            case Dict.get taskId model.blocks of
                Nothing ->
                    []

                Just taskIds ->
                    Set.foldl
                        (\blockedTaskId acc ->
                            case Dict.get blockedTaskId model.tasks of
                                Nothing ->
                                    acc

                                Just task ->
                                    task.title :: acc
                        )
                        []
                        taskIds
    in
    taskQuery
        |> Maybe.map
            (\task ->
                { title = task.title
                , description = task.description
                , size = task.size
                , created = task.created
                , updated = task.updated
                , due = task.due
                , tags = task.tags
                , assignees = assignees
                , blocks = blocks
                , blockedBy = blockedBy
                , expanded =
                    Set.memberOf model.expandedTasks taskId
                , taskId = taskId
                }
            )


taskViews : List TaskId -> Model -> List TaskView
taskViews ids model =
    List.foldl
        (\id acc ->
            case taskView id model of
                Nothing ->
                    acc

                Just task ->
                    task :: acc
        )
        []
        ids


emptyNewTask : NewTask r -> NewTask r
emptyNewTask newTask =
    { newTask
        | newTaskTitle = ""
        , newTaskDescription = ""
        , newTaskAssignees = []
        , newTaskSize = Small
        , newTaskBlocks = []
        , newTaskBlockedBy = []
        , newTaskDue = Nothing
        , newTaskTags = []
        , newTaskStatus = ToDo
        , newTaskUserSearchBox = SearchBox.reset newTask.newTaskUserSearchBox
        , newTaskUserSearchBoxText = ""
        , newTaskBlocksSearchBox = SearchBox.reset newTask.newTaskBlocksSearchBox
        , newTaskBlocksSearchBoxText = ""
        , newTaskBlockedBySearchBox = SearchBox.reset newTask.newTaskBlocksSearchBox
        , newTaskBlockedBySearchBoxText = ""
    }


taskFromNewTask : Posix -> NewTask r -> Task
taskFromNewTask time newTask =
    { title = newTask.newTaskTitle
    , description = newTask.newTaskDescription
    , assignees = newTask.newTaskAssignees
    , size = newTask.newTaskSize
    , created = time
    , updated = time
    , due = newTask.newTaskDue
    , tags = newTask.newTaskTags
    }


insertNewTaskIntoToDo : Posix -> Model -> Model
insertNewTaskIntoToDo time model =
    let
        taskId =
            TaskId (Dict.size model.tasks)
    in
    emptyNewTask
        { model
            | tasks = Dict.insert taskId (taskFromNewTask time model) model.tasks
            , toDo = taskId :: model.toDo
            , blocks = List.foldl (insertIntoSet taskId) model.blocks model.newTaskBlockedBy
            , blockedBy = List.foldl (insertIntoSet taskId) model.blockedBy model.newTaskBlocks
        }


insertNewTaskIntoInProgress : Posix -> Model -> Model
insertNewTaskIntoInProgress time model =
    let
        taskId =
            TaskId (Dict.size model.tasks)
    in
    emptyNewTask
        { model
            | tasks = Dict.insert taskId (taskFromNewTask time model) model.tasks
            , inProgress = taskId :: model.inProgress
            , blocks = List.foldl (insertIntoSet taskId) model.blocks model.newTaskBlockedBy
            , blockedBy = List.foldl (insertIntoSet taskId) model.blockedBy model.newTaskBlocks
        }


insertNewTaskIntoDone : Posix -> Model -> Model
insertNewTaskIntoDone time model =
    let
        taskId =
            TaskId (Dict.size model.tasks)
    in
    emptyNewTask
        { model
            | tasks = Dict.insert taskId (taskFromNewTask time model) model.tasks
            , done = taskId :: model.done
            , blocks = List.foldl (insertIntoSet taskId) model.blocks model.newTaskBlockedBy
            , blockedBy = List.foldl (insertIntoSet taskId) model.blockedBy model.newTaskBlocks
        }


insertIntoSet : TaskId -> key -> Dict key (Set TaskId) -> Dict key (Set TaskId)
insertIntoSet value key dict =
    case Dict.get key dict of
        Nothing ->
            let
                sorter =
                    Sort.by (\(TaskId id) -> id) Sort.increasing
            in
            Dict.insert key (Set.singleton sorter value) dict

        Just values ->
            Dict.insert key (Set.insert value values) dict


getAll : List key -> Dict key value -> List value
getAll keys dict =
    List.foldl
        (\key results ->
            case Dict.get key dict of
                Nothing ->
                    results

                Just value ->
                    value :: results
        )
        []
        keys


init : flags -> ( Model, Cmd msg )
init _ =
    ( { mode = ViewingBoard
      , users =
            Dict.empty (Sort.by (\(UserId id) -> id) Sort.increasing)

      -- Tasks
      , tasks =
            Dict.empty (Sort.by (\(TaskId id) -> id) Sort.increasing)
      , blocks =
            Dict.empty (Sort.by (\(TaskId id) -> id) Sort.increasing)
      , blockedBy =
            Dict.empty (Sort.by (\(TaskId id) -> id) Sort.increasing)
      , expandedTasks =
            Set.empty (Sort.by (\(TaskId id) -> id) Sort.increasing)

      -- Board
      , toDo = []
      , inProgress = []
      , done = []

      -- New Task
      , newTaskTitle = ""
      , newTaskDescription = ""
      , newTaskAssignees = []
      , newTaskSize = Small
      , newTaskBlocks = []
      , newTaskBlockedBy = []
      , newTaskDue = Nothing
      , newTaskTags = []
      , newTaskStatus = ToDo
      , newTaskUserSearchBox = SearchBox.init
      , newTaskUserSearchBoxText = ""
      , newTaskBlocksSearchBox = SearchBox.init
      , newTaskBlocksSearchBoxText = ""
      , newTaskBlockedBySearchBox = SearchBox.init
      , newTaskBlockedBySearchBoxText = ""

      -- New User
      , newUserName = ""
      }
    , Cmd.none
    )


type
    Msg
    -- Add Task
    = ClickedAddTask
    | ClickedAddTaskDone
    | ClickedAddTaskBack
    | GotNewTimeAfterAddTaskDone Posix
    | UpdatedNewTaskTitle String
    | UpdatedNewTaskDescription String
    | UpdatedNewTaskSize TaskSize
    | UpdatedNewTaskStatus TaskStatus
    | UpdatedNewTaskAssignees (SearchBox.ChangeEvent ( UserId, User ))
    | UpdatedNewTaskBlocks (SearchBox.ChangeEvent ( TaskId, Task ))
    | UpdatedNewTaskBlockedBy (SearchBox.ChangeEvent ( TaskId, Task ))
      -- Collapse Task
    | ClickedCollapseTask TaskId
    | ClickedExpandTask TaskId
      -- Add User
    | ClickedAddUser
    | ClickedAddUserDone
    | ClickedAddUserBack
    | UpdatedNewUserName String


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ClickedAddTask ->
            ( { model | mode = AddingTask }, Cmd.none )

        ClickedAddTaskDone ->
            ( { model | mode = ViewingBoard }
            , Task_.perform GotNewTimeAfterAddTaskDone Time.now
            )

        ClickedAddTaskBack ->
            ( { model | mode = ViewingBoard }, Cmd.none )

        GotNewTimeAfterAddTaskDone time ->
            case model.newTaskStatus of
                ToDo ->
                    ( insertNewTaskIntoToDo time model, Cmd.none )

                InProgress ->
                    ( insertNewTaskIntoInProgress time model, Cmd.none )

                Done ->
                    ( insertNewTaskIntoDone time model, Cmd.none )

        UpdatedNewTaskTitle title ->
            ( { model | newTaskTitle = title }, Cmd.none )

        UpdatedNewTaskDescription description ->
            ( { model | newTaskDescription = description }, Cmd.none )

        UpdatedNewTaskSize size ->
            ( { model | newTaskSize = size }, Cmd.none )

        UpdatedNewTaskStatus status ->
            ( { model | newTaskStatus = status }, Cmd.none )

        UpdatedNewTaskAssignees changeEvent ->
            case changeEvent of
                SearchBox.SelectionChanged ( userId, _ ) ->
                    ( { model
                        | newTaskAssignees = userId :: model.newTaskAssignees
                        , newTaskUserSearchBoxText = ""
                      }
                    , Cmd.none
                    )

                SearchBox.TextChanged text ->
                    ( { model
                        | newTaskUserSearchBoxText = text
                        , newTaskUserSearchBox = SearchBox.reset model.newTaskUserSearchBox
                      }
                    , Cmd.none
                    )

                SearchBox.SearchBoxChanged subMsg ->
                    ( { model
                        | newTaskUserSearchBox = SearchBox.update subMsg model.newTaskUserSearchBox
                      }
                    , Cmd.none
                    )

        UpdatedNewTaskBlocks changeEvent ->
            case changeEvent of
                SearchBox.SelectionChanged ( taskId, _ ) ->
                    ( { model
                        | newTaskBlocks = taskId :: model.newTaskBlocks
                        , newTaskBlocksSearchBoxText = ""
                      }
                    , Cmd.none
                    )

                SearchBox.TextChanged text ->
                    ( { model
                        | newTaskBlocksSearchBoxText = text
                        , newTaskBlocksSearchBox = SearchBox.reset model.newTaskBlocksSearchBox
                      }
                    , Cmd.none
                    )

                SearchBox.SearchBoxChanged subMsg ->
                    ( { model
                        | newTaskBlocksSearchBox = SearchBox.update subMsg model.newTaskBlocksSearchBox
                      }
                    , Cmd.none
                    )

        UpdatedNewTaskBlockedBy changeEvent ->
            case changeEvent of
                SearchBox.SelectionChanged ( taskId, _ ) ->
                    ( { model
                        | newTaskBlockedBy = taskId :: model.newTaskBlockedBy
                        , newTaskBlockedBySearchBoxText = ""
                      }
                    , Cmd.none
                    )

                SearchBox.TextChanged text ->
                    ( { model
                        | newTaskBlockedBySearchBoxText = text
                        , newTaskBlockedBySearchBox = SearchBox.reset model.newTaskBlockedBySearchBox
                      }
                    , Cmd.none
                    )

                SearchBox.SearchBoxChanged subMsg ->
                    ( { model
                        | newTaskBlockedBySearchBox = SearchBox.update subMsg model.newTaskBlockedBySearchBox
                      }
                    , Cmd.none
                    )

        ClickedAddUser ->
            ( { model | mode = AddingUser }, Cmd.none )

        ClickedAddUserDone ->
            let
                userId =
                    UserId (Dict.size model.users)
            in
            ( { model
                | mode = ViewingBoard
                , users = Dict.insert userId { name = model.newUserName } model.users
                , newUserName = ""
              }
            , Cmd.none
            )

        ClickedAddUserBack ->
            ( { model | mode = ViewingBoard }, Cmd.none )

        UpdatedNewUserName name ->
            ( { model | newUserName = name }, Cmd.none )

        ClickedCollapseTask taskId ->
            ( { model | expandedTasks = Set.remove taskId model.expandedTasks }, Cmd.none )

        ClickedExpandTask taskId ->
            ( { model | expandedTasks = Set.insert taskId model.expandedTasks }, Cmd.none )


view : Model -> Html Msg
view model =
    case model.mode of
        ViewingBoard ->
            Element.layout
                []
                (Element.column
                    []
                    [ viewBoard model
                    , addTaskButton
                    , addUserButton
                    ]
                )

        AddingTask ->
            Element.layout
                []
                (Element.column
                    []
                    [ Input.button
                        []
                        { onPress = Just ClickedAddTaskBack
                        , label = text "Back"
                        }
                    , Input.text
                        []
                        { onChange = UpdatedNewTaskTitle
                        , text = model.newTaskTitle
                        , placeholder = Nothing
                        , label = Input.labelAbove [] (text "Title")
                        }
                    , Input.text
                        []
                        { onChange = UpdatedNewTaskDescription
                        , text = model.newTaskDescription
                        , placeholder = Nothing
                        , label = Input.labelAbove [] (text "Description")
                        }
                    , Input.radioRow
                        []
                        { onChange = UpdatedNewTaskSize
                        , options =
                            [ Input.option Small (text "small")
                            , Input.option Medium (text "medium")
                            , Input.option Large (text "large")
                            ]
                        , selected = Just model.newTaskSize
                        , label = Input.labelAbove [] (text "Task Size")
                        }
                    , Input.radioRow
                        []
                        { onChange = UpdatedNewTaskStatus
                        , options =
                            [ Input.option ToDo (text "To Do")
                            , Input.option InProgress (text "In Progress")
                            , Input.option Done (text "Done")
                            ]
                        , selected = Just model.newTaskStatus
                        , label = Input.labelAbove [] (text "Task Status")
                        }
                    , viewNewTaskAssignees model.users model.newTaskAssignees
                    , SearchBox.input
                        []
                        { onChange = UpdatedNewTaskAssignees
                        , text = model.newTaskUserSearchBoxText
                        , selected = Nothing
                        , options =
                            model.newTaskAssignees
                                |> List.foldl Dict.remove model.users
                                |> Dict.toList
                                |> Just
                        , label = Input.labelAbove [] (text "Assignees")
                        , placeholder = Nothing
                        , toLabel = \( _, user ) -> user.name
                        , filter = \query ( _, { name } ) -> String.startsWith query name
                        , state = model.newTaskUserSearchBox
                        }
                    , viewTaskRow model.tasks model.newTaskBlocks
                    , SearchBox.input
                        []
                        { onChange = UpdatedNewTaskBlocks
                        , text = model.newTaskBlocksSearchBoxText
                        , selected = Nothing
                        , options =
                            List.append model.newTaskBlocks model.newTaskBlockedBy
                                |> List.foldl Dict.remove model.tasks
                                |> Dict.toList
                                |> Just
                        , label = Input.labelAbove [] (text "Blocks")
                        , placeholder = Nothing
                        , toLabel = \( _, task ) -> task.title
                        , filter = \query ( _, { title } ) -> String.contains query title
                        , state = model.newTaskBlocksSearchBox
                        }
                    , viewTaskRow model.tasks model.newTaskBlockedBy
                    , SearchBox.input
                        []
                        { onChange = UpdatedNewTaskBlockedBy
                        , text = model.newTaskBlockedBySearchBoxText
                        , selected = Nothing
                        , options =
                            List.append model.newTaskBlocks model.newTaskBlockedBy
                                |> List.foldl Dict.remove model.tasks
                                |> Dict.toList
                                |> Just
                        , label = Input.labelAbove [] (text "Blocked By")
                        , placeholder = Nothing
                        , toLabel = \( _, task ) -> task.title
                        , filter = \query ( _, { title } ) -> String.contains query title
                        , state = model.newTaskBlockedBySearchBox
                        }
                    , Input.button
                        []
                        { onPress = Just ClickedAddTaskDone
                        , label = text "Done"
                        }
                    ]
                )

        AddingUser ->
            Element.layout
                []
                (Element.column
                    []
                    [ Input.button
                        []
                        { onPress = Just ClickedAddUserBack
                        , label = text "Back"
                        }
                    , Input.text
                        []
                        { onChange = UpdatedNewUserName
                        , text = model.newUserName
                        , placeholder = Nothing
                        , label = Input.labelAbove [] (text "Name")
                        }
                    , Input.button
                        []
                        { onPress = Just ClickedAddUserDone
                        , label = text "Done"
                        }
                    ]
                )


viewBoard : Model -> Element Msg
viewBoard model =
    Element.row
        []
        [ model.toDo
            |> (\taskIds -> taskViews taskIds model)
            |> viewToDo
        , model.inProgress
            |> (\taskIds -> taskViews taskIds model)
            |> viewInProgress
        , model.done
            |> (\taskIds -> taskViews taskIds model)
            |> viewDone
        ]


viewToDo : List TaskView -> Element Msg
viewToDo tasks =
    Element.column
        []
        (List.map viewTask tasks)


viewInProgress : List TaskView -> Element Msg
viewInProgress tasks =
    Element.column
        []
        (List.map viewTask tasks)


viewDone : List TaskView -> Element Msg
viewDone tasks =
    Element.column
        []
        (List.map viewTask tasks)


addTaskButton : Element Msg
addTaskButton =
    Input.button
        []
        { onPress = Just ClickedAddTask
        , label = text "Add Task"
        }


addUserButton : Element Msg
addUserButton =
    Input.button
        []
        { onPress = Just ClickedAddUser
        , label = text "Add User"
        }


viewNewTaskAssignees : Dict UserId User -> List UserId -> Element Msg
viewNewTaskAssignees users userIds =
    let
        assigned =
            getAll userIds users
    in
    Element.row
        []
        (List.map (.name >> text) assigned)


viewTaskRow : Dict TaskId Task -> List TaskId -> Element Msg
viewTaskRow tasks taskIds =
    let
        blocks =
            getAll taskIds tasks
    in
    Element.row
        []
        (List.map (.title >> text) blocks)


viewTask : TaskView -> Element Msg
viewTask task =
    if task.expanded then
        Element.column
            []
            [ Element.row
                []
                [ Input.button
                    []
                    { onPress = Just (ClickedCollapseTask task.taskId)
                    , label = text "V"
                    }
                , Element.text task.title
                ]
            , Element.text task.description
            , Element.text (sizeToString task.size)
            , Element.row [] (List.map text task.assignees)
            , Element.row [] (List.map text task.blocks)
            , Element.row [] (List.map text task.blockedBy)
            ]

    else
        Element.row
            []
            [ Input.button
                []
                { onPress = Just (ClickedExpandTask task.taskId)
                , label = text ">"
                }
            , Element.text task.title
            ]


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = \_ -> Sub.none
        }
