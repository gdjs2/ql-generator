# LLM Assissted Program Analysis (Temporary Name)

This project is dedicated to using Large Language Model (LLM) to complement traditional program analysis. With our program, you can use the Large Language Model to help understand the semantics of a program while using traditional program analysis tools, thus making the analysis more complete. 

Our project is divided into three main parts as follows:

1. Extractor: extract the required content from the project for analysis.
2. Engine: ask LLM to get the information you need.
3. Generator: generate files to aid in program analysis.

## Current Implementation

Currently, our implementation is based on CodeQL and ChatGPT:
1. Extractor: input is a CodeQL Database. The extractor will extract the functions from the database.
2. Engine: 

