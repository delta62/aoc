module Day04Test (tests) where

import Day04 (part1, part2)
import Test.HUnit
import Text.Printf (printf)

readExample :: String -> IO String
readExample s = readFile $ printf "examples/%s.txt" s

day4Test :: (String -> Assertion) -> Test
day4Test f = TestCase $ readExample "day04" >>= f

part1Test :: String -> Assertion
part1Test input = 18 @=? part1 input

part2Test :: String -> Assertion
part2Test input = 9 @=? part2 input

tests :: Test
tests = TestLabel "Day 4" $ TestList allTests
  where
    allTests = map day4Test [part1Test, part2Test]
