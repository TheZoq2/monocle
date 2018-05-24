module Model exposing (Model, init, MouseDragReceiver(..))

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
    , lastDragPos: (Float, Float)
    , graphOffset: Float
    }


init : (Model, Cmd Msg)
init =
    ( { readings = initialReadings
      , currentReading = (Reading [False, False] 400)
      , triggerMode = FallingEdge
      , timeSpan = Time Millisecond 1
      , triggerChannel = 1
      , mouseDragReceiver = Nothing
      , lastDragPos = (0,0)
      , graphOffset = 0
    }
    , Cmd.none
    )


initialReadings : List Reading
initialReadings =
    [ Reading [False, False] 0
    , Reading [True, False] 100
    , Reading [False, True] 300
    , Reading [True, True] 350
    ]
