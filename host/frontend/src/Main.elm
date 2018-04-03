module Main exposing (..)

import WebSocket
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)

type alias Reading =
    { time: Int
    , value: Int
    }




-- Model and init

type alias Model =
    { readings: List Reading
    }


init : (Model, Cmd Msg)
init =
    ( { readings = [] }
    , Cmd.none
    )





-- Messages

type Msg
    = NewMessage String





-- Update function

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    (model, Cmd.none)



-- Subscriptions

subscriptions : Model -> Sub Msg
subscriptions model = 
    WebSocket.listen "localhost:8765" NewMessage


-- View

view : Model -> Html Msg
view model =
    p [] [text "hello world"]



main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }
