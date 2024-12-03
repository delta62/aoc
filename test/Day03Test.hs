module Day03Test (tests) where

import Test.HUnit
import Text.Printf (printf)
import Day03 (part1, part2)

readExample :: String -> IO String
readExample s = readFile $ printf "examples/%s.txt" s

day3Test :: (String -> Assertion) -> Test
day3Test f = TestCase $ readExample "day03" >>= f

part1Test :: String -> Assertion
part1Test input = 161 @=? part1 input

part2Test :: String -> Assertion
part2Test input = 48 @=? part2 input

tests :: Test
tests = TestLabel "Day 3" $ TestList allTests
  where
    allTests = map day3Test [part1Test]
