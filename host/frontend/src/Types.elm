module Types exposing (Reading, readingDecoder)

import Json.Decode as De
import List.Extra

type alias Reading =
    { values: List Bool
    , time: Float
    }


readingDecoder : De.Decoder Reading
readingDecoder =
    De.map2 Reading
        (De.field "values" (De.list De.bool))
        (De.field "time" De.float)


readingsToChannels : List Reading -> List (List (Float, Bool))
readingsToChannels readings =
    let
        readingsLists = List.map (\reading -> reading.values) readings
        timeList = List.map (\reading -> reading.time) readings
    in
        List.Extra.transpose readingsLists
            |> List.map (List.Extra.zip timeList)



