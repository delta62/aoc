module Main where

import Test.HUnit
import qualified System.Exit as Exit
import Day01 (part1, part2)

input :: String
input = unlines [
    "3   4",
    "4   3",
    "2   5",
    "1   3",
    "3   9",
    "3   3"
    ]

test1 :: Test
test1 = TestCase (assertEqual "for part 1" 11 (part1 input))

test2 :: Test
test2 = TestCase (assertEqual "for part 2" 31 (part2 input))

tests :: Test
tests = TestLabel "Day 1" $ TestList [test1, test2]

main :: IO ()
main = do
    result <- runTestTT tests
    if failures result > 0 then Exit.exitFailure else Exit.exitSuccess
