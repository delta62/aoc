module Main where

import Day01 (part1, part2)

main :: IO ()
main = do
    input <- readFile "input/day01.txt"
    putStrLn $ "Part 1: " ++ (show $ part1 input)
    putStrLn $ "Part 2: " ++ (show $ part2 input)
