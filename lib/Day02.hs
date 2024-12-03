module Day02 where

import Util (parseInt, mapAdjacent, count)

diffList :: [Int] -> [Int]
diffList = mapAdjacent (-)

validPositive :: Int -> Bool
validPositive x = x `elem` [1..3]

validNegative :: Int -> Bool
validNegative x = x `elem` [(-3)..(-1)]

dropAt :: Int -> [a] -> [a]
dropAt n xs =
    let (ys,zs) = splitAt n xs
     in ys ++ (tail zs)

isKindaSafe :: [a] -> [[a]]
isKindaSafe xs =
    let len = pred $ length xs
     in [dropAt n xs | n <- [0..len]]

isSafe :: [Int] -> Bool
isSafe xs = allPositive || allNegative
  where
    diffs = diffList xs
    allPositive = all validPositive diffs
    allNegative = all validNegative diffs

isSafe' :: [Int] -> Bool
isSafe' xs = isSafe xs || any isSafe (isKindaSafe xs)

parse :: String -> [[Int]]
parse = (map parseLine) . lines

parseLine :: String -> [Int]
parseLine = (map parseInt) . words

part1 :: String -> Int
part1 = (count isSafe) . parse

part2 :: String -> Int
part2 = (count isSafe') . parse
