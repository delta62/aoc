module Main where

import Test.HUnit
import qualified System.Exit as Exit
import qualified Day01Test as D01

tests :: Test
tests = D01.tests

main :: IO ()
main = do
    result <- runTestTT tests
    if failures result > 0 then Exit.exitFailure else Exit.exitSuccess
