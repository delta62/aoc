module Main where

import Test.HUnit
import qualified System.Exit as Exit
import qualified Data.Map as M
import Text.Printf (printf)
import Day01 (parse, parse', part1, part2)

readExample :: String -> IO String
readExample s = readFile $ printf "examples/%s.txt" s

day1Test :: (String -> Assertion) -> Test
day1Test f = TestCase $ readExample "day01" >>= f

parseTest :: String -> Assertion
parseTest input = parse input @=? (list1, list2)
  where
    list1 = [3, 4, 2, 1, 3, 3]
    list2 = [4, 3, 5, 3, 9, 3]

parse'Test :: String -> Assertion
parse'Test input = parse' input @=? (keys, hash)
  where
    keys = [3, 4, 2, 1, 3, 3]
    hash = M.fromList [(3, 3), (4, 1), (5, 1), (9, 1)]

part1Test :: String -> Assertion
part1Test input = 11 @=? part1 input

part2Test :: String -> Assertion
part2Test input = 31 @=? part2 input

tests :: Test
tests = TestLabel "Day 1" $ TestList $ map day1Test [parseTest, parse'Test, part1Test, part2Test]

main :: IO ()
main = do
    result <- runTestTT tests
    if failures result > 0 then Exit.exitFailure else Exit.exitSuccess
