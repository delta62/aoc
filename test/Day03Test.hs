module Day03Test (tests) where

import Day03 (part1, part2)
import Test.HUnit
import Text.Printf (printf)

readExample :: String -> IO String
readExample s = readFile $ printf "examples/%s.txt" s

day3Test :: (String -> Assertion) -> Test
day3Test f = TestCase $ readExample "day03" >>= f

day3p2Test :: (String -> Assertion) -> Test
day3p2Test f = TestCase $ readExample "day03_part2" >>= f

part1Test :: String -> Assertion
part1Test input = 161 @=? part1 input

part2Test :: String -> Assertion
part2Test input = 48 @=? part2 input

multiLineTest :: Test
multiLineTest = TestCase $ 4 @=? part2 "don't()\nmul(3,3)\ndo()\nmul(2,2)"

tests :: Test
tests = TestLabel "Day 3" $ TestList allTests
  where
    allTests = [day3Test part1Test, day3p2Test part2Test, multiLineTest]
