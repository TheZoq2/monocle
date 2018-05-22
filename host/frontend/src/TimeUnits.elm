module TimeUnits exposing 
    ( TimeUnit(..)
    , Time
    , toMicroseconds
    , allTimeUnits
    , timeUnitString
    )

type TimeUnit
    = Second
    | Millisecond
    | Microsecond
    | Nanosecond



type alias Time =
    { unit: TimeUnit
    , value: Float
    }


toMicroseconds : Time -> Float
toMicroseconds {unit, value} =
    case unit of
        Second -> value * 1000000
        Millisecond-> value * 1000
        Microsecond ->  value
        Nanosecond -> value / 1000


allTimeUnits : List TimeUnit
allTimeUnits =
    [Second, Millisecond, Microsecond, Nanosecond]

timeUnitString : TimeUnit -> String
timeUnitString unit =
    case unit of
        Second -> "s"
        Millisecond -> "ms"
        Microsecond -> "μs"
        Nanosecond -> "ns"
