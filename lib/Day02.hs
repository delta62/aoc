module Day02 where

import Util (parseInt, mapAdjacent, count)

diffList :: [Int] -> [Int]
diffList = mapAdjacent (-)

validPositive :: Int -> Bool
validPositive x = x >= 1 && x <= 3

validNegative :: Int -> Bool
validNegative x = x >= -3 && x <= -1

isSafe :: [Int] -> Bool
isSafe xs = allPositive || allNegative
  where
    diffs = diffList xs
    allPositive = all validPositive diffs
    allNegative = all validNegative diffs

parse :: String -> [[Int]]
parse = (map parseLine) . lines

parseLine :: String -> [Int]
parseLine = (map parseInt) . words

part1 :: String -> Int
part1 = (count isSafe) . parse
