module Main where

import Test.HUnit
import qualified System.Exit as Exit
import qualified Day01Test as D01
import qualified Day02Test as D02
import qualified Day03Test as D03

tests :: Test
tests = TestList [ D01.tests, D02.tests, D03.tests ]

main :: IO ()
main = do
    result <- runTestTT tests
    if failures result > 0 then Exit.exitFailure else Exit.exitSuccess
