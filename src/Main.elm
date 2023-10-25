module Main exposing (main)

import Browser
import Element exposing (Element, text)
import Element.Input as Input
import Html exposing (Html)
import SearchBox
import Sort
import Sort.Dict as Dict exposing (Dict)
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



-- type TaskBundle = TaskBundle (List TaskBlueprint)


type Mode
    = ViewingBoard
    | AddingTask
    | AddingUser


type Status
    = ToDo
    | InProgress
    | Done


type alias User =
    { name : String
    }


type alias Task =
    { title : String
    , description : String
    , assignees : List UserId
    , size : TaskSize
    , blocks : List TaskId
    , blockedBy : List TaskId
    , created : Posix
    , updated : Posix
    , due : Maybe Posix
    , tags : List TagId
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
        , newTaskStatus : Status
        , newTaskUserSearchBox : SearchBox.State
        , newTaskUserSearchBoxText : String
        , newTaskBlocksSearchBox : SearchBox.State
        , newTaskBlocksSearchBoxText : String
    }


type alias Model =
    { tasks : Dict TaskId Task
    , users : Dict UserId User
    , mode : Mode

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
    , newTaskStatus : Status
    , newTaskUserSearchBox : SearchBox.State
    , newTaskUserSearchBoxText : String
    , newTaskBlocksSearchBox : SearchBox.State
    , newTaskBlocksSearchBoxText : String

    -- New User
    , newUserName : String
    }


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
    }


insertToDoTask : TaskId -> Task -> Model -> Model
insertToDoTask taskId task model =
    { model
        | tasks = Dict.insert taskId task model.tasks
        , toDo = taskId :: model.toDo
    }


insertInProgressTask : TaskId -> Task -> Model -> Model
insertInProgressTask taskId task model =
    { model
        | tasks = Dict.insert taskId task model.tasks
        , inProgress = taskId :: model.toDo
    }


insertDoneTask : TaskId -> Task -> Model -> Model
insertDoneTask taskId task model =
    { model
        | tasks = Dict.insert taskId task model.tasks
        , done = taskId :: model.toDo
    }


getAll : List key -> Dict key value -> List value
getAll keys dict =
    List.foldr
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
    ( { tasks =
            Dict.empty (Sort.by (\(TaskId id) -> id) Sort.increasing)
      , users =
            Dict.empty (Sort.by (\(UserId id) -> id) Sort.increasing)
      , mode = ViewingBoard

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
    | UpdatedNewTaskStatus Status
    | UpdatedNewTaskAssignees (SearchBox.ChangeEvent ( UserId, User ))
    | UpdatedNewTaskBlocks (SearchBox.ChangeEvent ( TaskId, Task ))
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
            let
                taskId =
                    TaskId (Dict.size model.tasks)

                task =
                    { title = model.newTaskTitle
                    , description = model.newTaskDescription
                    , assignees = model.newTaskAssignees
                    , size = model.newTaskSize
                    , blocks = model.newTaskBlocks
                    , blockedBy = model.newTaskBlockedBy
                    , created = time
                    , updated = time
                    , due = model.newTaskDue
                    , tags = model.newTaskTags
                    }
            in
            case model.newTaskStatus of
                ToDo ->
                    ( model
                        |> emptyNewTask
                        |> insertToDoTask taskId task
                    , Cmd.none
                    )

                InProgress ->
                    ( model
                        |> emptyNewTask
                        |> insertInProgressTask taskId task
                    , Cmd.none
                    )

                Done ->
                    ( model
                        |> emptyNewTask
                        |> insertDoneTask taskId task
                    , Cmd.none
                    )

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
                                |> List.foldr Dict.remove model.users
                                |> Dict.toList
                                |> Just
                        , label = Input.labelAbove [] (text "Assignees")
                        , placeholder = Nothing
                        , toLabel = \( _, user ) -> user.name
                        , filter = \query ( _, { name } ) -> String.startsWith query name
                        , state = model.newTaskUserSearchBox
                        }
                    , viewNewTaskBlocks model.tasks model.newTaskBlocks
                    , SearchBox.input
                        []
                        { onChange = UpdatedNewTaskBlocks
                        , text = model.newTaskBlocksSearchBoxText
                        , selected = Nothing
                        , options =
                            model.newTaskBlocks
                                |> List.foldr Dict.remove model.tasks
                                |> Dict.toList
                                |> Just
                        , label = Input.labelAbove [] (text "Blocks")
                        , placeholder = Nothing
                        , toLabel = \( _, task ) -> task.title
                        , filter = \query ( _, { title } ) -> String.contains query title
                        , state = model.newTaskBlocksSearchBox
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
        [ viewToDo (getAll model.toDo model.tasks)
        , viewInProgress (getAll model.inProgress model.tasks)
        , viewDone (getAll model.done model.tasks)
        ]


viewToDo : List Task -> Element Msg
viewToDo tasks =
    Element.column
        []
        (List.map viewTask tasks)


viewInProgress : List Task -> Element Msg
viewInProgress tasks =
    Element.column
        []
        (List.map viewTask tasks)


viewDone : List Task -> Element Msg
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


viewNewTaskBlocks : Dict TaskId Task -> List TaskId -> Element Msg
viewNewTaskBlocks tasks taskIds =
    let
        blocks =
            getAll taskIds tasks
    in
    Element.row
        []
        (List.map (.title >> text) blocks)


viewTask : Task -> Element Msg
viewTask task =
    Element.el [] (Element.text task.title)


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = \_ -> Sub.none
        }
