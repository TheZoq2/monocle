module Model exposing (Model, init)

import Types exposing (Reading, TriggerMode(..))
import TimeUnits exposing (Time, TimeUnit(..))
import Msg exposing (Msg)

-- Helper types
type MouseDragReceiver
    = Graph

-- Model and init

type alias Model =
    { readings: List Reading
    , currentReading: Reading
    , triggerMode: TriggerMode
    , timeSpan: Time
    , triggerChannel: Int
    , mouseDragReceiver: Maybe MouseDragReceiver
    }


init : (Model, Cmd Msg)
init =
    ( { readings = []
      , currentReading = (Reading [] 0)
      , triggerMode = FallingEdge
      , timeSpan = Time Millisecond 1
      , triggerChannel = 1
      , mouseDragReceiver = Nothing
    }
    , Cmd.none
    )


