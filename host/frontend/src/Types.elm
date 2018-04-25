module Types exposing
    ( Message(..)
    , Reading
    , messageDecoder
    , readingsToChannels
    , TriggerMode(..)
    , triggerModeSymbol
    , allTriggerModes
    )

import Json.Decode as De
import List.Extra

type alias Reading =
    { values: List Bool
    , time: Float
    }


type Message
    = CurrentTime Float
    | NewReading Reading


readingDecoder : De.Decoder Reading
readingDecoder =
    De.map2 Reading
        (De.field "values" (De.list De.bool))
        (De.field "time" De.float)


messageDecoder : De.Decoder Message
messageDecoder =
    let
        reading = De.map (\a -> NewReading a) <| De.field "Reading" readingDecoder
        currentTime = De.map (\a -> CurrentTime a) <| De.field "CurrentTime" De.float
    in
        De.oneOf [reading, currentTime]


readingsToChannels : List Reading -> List (List (Float, Bool))
readingsToChannels readings =
    let
        readingsLists = List.map (\reading -> reading.values) readings
        timeList = List.map (\reading -> reading.time) readings
    in
        List.Extra.transpose readingsLists
            |> List.map (List.Extra.zip timeList)



type TriggerMode
    = Continuous
    | FallingEdge
    | RisingEdge


triggerModeSymbol : TriggerMode -> String
triggerModeSymbol mode =
    case mode of
        Continuous -> "→"
        FallingEdge -> "↓"
        RisingEdge -> "↑"


allTriggerModes : List TriggerMode
allTriggerModes =
    [ Continuous
    , FallingEdge
    , RisingEdge
    ]

