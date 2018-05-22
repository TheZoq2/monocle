module Msg exposing (Msg(..))

import Types exposing
    ( Message(..)
    , Reading
    , TriggerMode(..)
    )

import Mouse

import TimeUnits exposing (TimeUnit(..))

type Msg
    = NewMessage String
    | TriggerModeSet TriggerMode
    | TimeSpanSet String
    | TimeSpanUnitSet TimeUnit
    | TriggerChannelSet Int
    | ResetValues
    | MouseGlobalMove Mouse.Position
    | MouseGlobalUp Mouse.Position
