# Structure and Interpretation of Compiler Problems: a foray into the design and implementation of programming languages

**By: Anton Filippov**

# Contents
- [Structure and Interpretation of Compiler Problems: a foray into the design and implementation of programming languages](#structure-and-interpretation-of-compiler-problems-a-foray-into-the-design-and-implementation-of-programming-languages)
- [Contents](#contents)
- [Foreword](#foreword)
- [Theory](#theory)
    - [High level overview](#high-level-overview)
    - [Language design](#language-design)
    - [Lexing](#lexing)
    - [Parsing](#parsing)
    - [Evaluating](#evaluating)
- [Practice](#practice)
    - [Lexer](#lexer)
    - [Parser](#parser)
    - [Evaluator](#evaluator)
- [Conclusion](#conclusion)
- [Bibliography](#bibliography)
- [Appendix](#appendix)

# Foreword

Software is one of the staples of the modern society as we know it. It permeates every inch of our lives as humans, from our work environments, to our entertainment, to the cities we live in, to the cars we drive - the list goes on and on. It is vitally important that the software that we create is robust and free of errors, as the opposite could in the worst case scenario lead to massively fatal consequences.

To be able to engineer high quality software systems, one must first be able to reason about them - that is, understand their structure and behavior. To that end, we employ various abstraction models to ease the burden of keeping the inherent complexity of a given system in check. For example, in an email application, we can say that we have a "username" and a "password", and an action called "login" that runs checks on user credentials. This maps closely to how we humans reason about the world - naming things and considering ways in which they relate to each other.

A computer, however, does not see the world in this way. Even today, computers by and large are "dumb" machines that only understand zeroes and ones. A computer doesn't know what a "password" is, or what the difference between "logging in" and "baking a cake" is, because none of these concepts exist at the level of abstraction where computers typically operate. So how does one communicate with a computer, then?

The way computers operate is by executing sequences of instructions defined and supported by their architecture, encoded in raw binary form. In a simplified model, each instruction can have a part that specifies which operation to perform, and zero, one or more parts that represent the operands to be used by this operation. For example, in the 80x86 architecture, the operation of "no operation" is represented by a number 0x90 with no operands. It is not inconceivable, then, to give this instruction a mnemonic, such as NOP. Now, instead of dealing with an arbitrary hard-to-remember number, we can work with something that has a name that tells us exactly what it does, at the added cost of having to translate all the names back into numbers before execution. In essence, we have just defined the most useless subset of assembly language in the world.

This process of translation forms the basis for all computer languages. Back in ye olden days, the translation needed to be done by hand. However, as time went on, it became apparent that certain kinds of languages lend themselves well to automated translation into a form suitable for a computer. This observation, coupled with other advances in what eventually became the field of computer languages, has made it possible to raise the apparent level of abstraction that a computer can understand, allowing not only for thin veneer of assembly language, but also much more expressive abstractions that have no direct equivalent in hardware, such as functions, type hierarchies, going all the way up to channels and actors, and many more.

In this study, I would like to shed some light into the science that makes the world tick. In part one, I will take a brief look at the history of programming languages. I will examine the different approaches taken by various languages to solve certain problems. Finally, I will dedicate some time to the discussion of Rust, my language of choice for the practical section of this study.

In part two, I will take a deep dive into the mechanisms behind modern programming languages. I'll start off by giving a high-level overview of the translation process. I will then talk in order about each of its stages, including lexing, parsing, parse tree manipulations, and execution.

Finally, in part three, I will make an attempt to apply in practice some of the techniques that I have gleaned during my research, by designing a small programming language and constructing a functioning interpreter for it. The acceptance metric for the interpreter will be to correctly execute a simple Fizzbuzz program.

# Theory

## High level overview

## Language design

## Lexing

## Parsing

## Evaluating

# Practice

## Lexer

## Parser

## Evaluator

# Conclusion

# Bibliography

# Appendix