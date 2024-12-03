{-# OPTIONS_GHC -Wno-incomplete-patterns #-}

module Day01 (parse, parse', part1, part2) where

import Data.List (sort)
import qualified Data.Map as M
import Util (parseInt)

type Lookup = M.Map Int Int

tuple :: [a] -> (a, a)
tuple [x, y] = (x, y)

parseLine :: String -> (Int, Int)
parseLine s = tuple $ map parseInt $ words s

parse :: String -> ([Int], [Int])
parse s = unzip $ map parseLine $ lines s

counts :: (Num a, Ord a, Enum a) => [a] -> M.Map a a
counts xs = foldr folder M.empty xs
  where
    inc _ x = succ x
    folder x acc = M.insertWith inc x 1 acc

parse' :: String -> ([Int], Lookup)
parse' s = (set, hash)
  where
    (set, h) = unzip $ map parseLine $ lines s
    hash = counts h

diffs :: ([Int], [Int]) -> Int
diffs (xs, ys) =
    let diff = abs . uncurry (-)
     in sum $ map diff $ zip (sort xs) (sort ys)

scores :: [Int] -> Lookup -> Int
scores ks m =
    let score k = k * (M.findWithDefault 0 k m)
     in sum $ map score ks

part1 :: String -> Int
part1 input = diffs $ parse input

part2 :: String -> Int
part2 input =
    let (keys, entries) = parse' input
     in scores keys entries
