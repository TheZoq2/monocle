module Types exposing (Reading, readingDecoder)

import Json.Decode as De

type alias Reading =
    { channel1: Bool
    , channel2: Bool
    , time: Float
    }


readingDecoder : De.Decoder Reading
readingDecoder =
    De.map3 Reading
        (De.field "channel1" De.bool)
        (De.field "channel2" De.bool)
        (De.field "time" De.float)

