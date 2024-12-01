module Main where

import Data.List (sort)

parseStr :: String -> Int
parseStr = read

tuple :: [a] -> (a, a)
tuple [x, y] = (x, y)

parseLine :: String -> (Int, Int)
parseLine s = tuple $ map parseStr $ words s

parse :: String -> ([Int], [Int])
parse s = unzip $ map parseLine (lines s)

minDiffs :: ([Int], [Int]) -> Int
minDiffs (xs, ys) = sum $ map (\(x, y) -> abs (x - y)) $ zip as bs
  where
    as = sort xs
    bs = sort ys

main :: IO ()
main = do
    input <- readFile "input/day01.txt"
    let answer = minDiffs $ parse input
    putStrLn $ show answer
