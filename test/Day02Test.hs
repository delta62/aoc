module Day02Test (tests) where

import Test.HUnit
import Text.Printf (printf)
import Day02 (part1, part2)

readExample :: String -> IO String
readExample s = readFile $ printf "examples/%s.txt" s

day2Test :: (String -> Assertion) -> Test
day2Test f = TestCase $ readExample "day02" >>= f

part1Test :: String -> Assertion
part1Test input = 2 @=? part1 input

part2Test :: String -> Assertion
part2Test input = 4 @=? part2 input

tests :: Test
tests = TestLabel "Day 2" $ TestList allTests
  where
    allTests = map day2Test [part1Test, part2Test]
