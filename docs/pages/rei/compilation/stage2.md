---
layout: page
title: Stage 2 - Syntax Analysis
parent: Compilation
grand_parent: Rei
nav_order: 2
---

## Overview

This is the second major part of a compiler 'frontend' that actually tries to build meaning from tokens and attributes. A lot of this is based on ordering, tree structuring, etc.

The lexical analyser sends tokens to the parser, which forms a parse tree. When the entire sequence has been processed, so should the parse tree. Assume the output of a parser is always a parse tree like structure.

- the parse tree is useful since we can use it to form a more generalised, lower level IR of the source code. This IR can expand on stuff to make everything explicit as possible and be generalised enough to do analysis on, i.e. optimisation steps
- each step of the frontend usually has access to the global symbol table formed in the lexical analyser

## Grammars

Type 1 -> Universal \
Type 2 -> Top-Down \
Type 3 -> Bottom-Up

In most cases, a bottom-up grammar is better since the parsers for them are more efficient.

For universal parsing, we can use CYK or Earley's algorithm. These are quite inefficient, so much so that they arent really practical in real life situations.

If we want a really efficient parser (TD or BU), they usually only work for certain classes of grammars. Namely LL and LR.

- these grammars are expressive enough to describe most of the syntax/nesting in most modern languages. So they dont seem like a bad choice
- LL can be implemented by hand
- LR are ormally generated using automated tools

### Representative Grammars

Constructs that begin with `while`, `if`, and other "structured" keywords are usually easier to parse. But expressions that involve identifiers and operators are more of a problem

- operators should have a precedence, like `()` before everything else, `*` and `/` before `+` and `-`, etc.

We can specify a list of 'rules' for expressions:

```
E -> E + T | T
T -> T * F | F
F -> (E) | id
```

- these rules describe the associativity and precedence of terms. `(expression)` expressions are prioritised. `*` expressions are next in line. And `+` are the least priority
- note how the priority is built from `F` and `T` then `E`. If we look at it from top-bottom, it is recursive. This means the grammar is an LR grammar. LR grammars can be parsed bottom-up

For a top-down grammar, we dont use recursion. Instead we specify extra rules:

```
E -> T E'
E' -> + T E' | epsilon
T -> F T'
T' -> *F T' | epsilon
F -> (E) | id 
```

- note how the grammars are all 'circular'. `F` relies on `E` and vice-versa

## Errors in Syntax

Compilers cant expect the input to be always correct. Humans make mistakes. Many mistakes. The compiler should even expect some common mistakes (e.g. rust) and suggest ways to correct them. But at least they should be able to spot a syntax error and throw some kind of meaningful error message about the problem

- its a good idea to plan how to tackle syntax errors. Prob better if you know exactly how your language looks, edge case testing, and common errors

### Types of Errors

Lexical -> misspellings of identifiers, keywords, operators

Syntactic -> misplaced semicolons, extra/missing braces and keywords

Semantic -> mismatch between operators and operands. Prob a bit harder to analyse off the bat. If you are returning an `Int` but the return type is actually `none`, then you have a syntax error

- one way is to not specify a return type sometimes

Logical -> incorrect reasoning, e.g. using the `=` operator when they actually wanted `==`. The end program may compile OK but the logic might not work as expected. These errors seem hardest to detect as they arent much to do with program validity but intention accuracy. Tests and etc should help and also a parser that checks for common things like this and raises a warning

Syntactic errors are usually easily detectable with usual parsing methods. LL and LR methods detect errors as soon as possible, if the stream of tokens do not match the grammar, then something is wrong

- usually it makes sense to just panic, print out the line and surrounding context where an error was detected, and quit

Goals of an error handler:

- report presence of errors clearly and accurately
- recover from each error quickly enough to detect subsequent errors
- minimum overhead when processing correct programs

## Context Free Grammars

The formal definition of a CFG:

- a grammar that contains terminals, nonterminals, start symbols, productions
- terminals = symbols from which strings are formed
- non-terminals = syntactic variables that represent sets of strings. A statement or expression is a nonterminal. Imposes a hierarchical structure on the language -> parse tree
- start symbol = a way to distinguish a certain nonterminal. The set of strings the nonterminal represents is the langauge generated by the grammar (lexeme). "Productions" for start symbols should be listed first
- productions = a way to specify the manner in which the terminals and nonterminals can be combined to form strings

A production consists of:

- a non terminal "head" or "left" of the production. This defines some of the strings represented by the head
- the symbol `->`
- a "body" or "right" of the production. Consists of zero or more terminals and non terminals. Describes one way in which strings of the nonterminal at the head can be constructed

### Notations

Terminals are either:

- lowercase letters like `a`
- operators like `+`
- punctuation like `,`
- digits `0, 1 ...`
- bolded strings like `id`, `if`. Each of these represents a single terminal symbol

Nonterminals are:

- uppercase letters like `A`
- `S`, which is the start symbol for a production
- lowercase and italicised strings like `expr`, `stmt`

Note, uppercase letters may also be used to represent more generic programming constructions like expressions, terms and factors

Also:

- Later uppercase `X, Y, Z` are usually used to represent grammar symbols like nonterminals and terminals. So they are metasymbol
- Later lowercase `x, y, z` usually represent possibly empty strings of terminals
- Lowercase greek letters like alpha are used to represent strings of grammar symbols. E.g. `A -> alpha` which means head -> body
- Productions like `A -> alpha_1` can be written with a common head `A -> alpha_1, alpha_2 ... alpha_k`
- Usually the head of the first production is the start symbol, regardless of whether it is `S` or something else

Example:

```
E -> E + T | E - T | T
T -> T * F | T / F | F
F -> (E) | id
```

- Nonterminals: E, T, F. There are 3 productions. All nonterminals are recursive and have circular dependencies
- E is the start symbol
- In the 1st production, we have 5 nonterminals. There are 2 terminals (operators `+` and `-`)
- In the 2nd production, we also have 5 nonterminals. Tere are also 2 terminals (operators `*` and `/`)
- In the 3rd production, we have a nonterminal `E` and a terminal `id`
- Note how the terminals can be directly parsed while the nonterminals need more context to be directly parsed into lexemes

### Derivations

We can treat productions as rewriting rules.

- We start at the start symbol of a production. Then we apply a bunch of rewrite steps
- Each rewrite step replaces a nonterminal with the body of one of it's productions
- For bottom up parsing, we use something called "rightmost derivations"

Example grammar:

```
E -> E + E | E * E | -E | (E) | id
```

- `-E` means if `E` is an expression, then `-E` is also an expression. Kind of like negation of the expression
- `(E)` is applied to replace any instance of `E` by `(E)` like `E * E => (E) * E`. It seems weird but is quite useful

This means with the above grammar for any expression `E`, we can do:

```
E => -E => -(E) => -(id)
```

- we have derived `-(id)` from `E`. Meaning a string `-(id)` is an instance of an expression
- this is important for seeing whether a grammar is context free and parseable by bottom-up parsing algorithms

We use `=>*` to mean 'derives in zero or more steps' and `=>+` to mean `derives in one or more steps`.

- if `S =>* alpha`, that means `alpha` is a sequential form of the grammar `G`. Given `S` is the start symbol for `G`

### Context Free Derivation

A sequential form may contain both terminals + nonterminals, or could be empty (epsilon). A sentence of `G` is a sequential form with no nonterminals, so it is completely self-contained/direct.

- the language generated by a grammar is its full set of sentences. So a string of terminals `omega` is in L(G) iff `omega` is a sentence of G. I.e. `S =>* w`
- this also means `w` can be derived from the grammar through as many steps to derive as possible. It just has to get there eventually. We can see how formal validation of an entire source code file could work

Hence: A language that can be generated by a grammar is `context-free`.

- if two grammars generate the same language (result in the same set of sentences), then they are equivalent/identical

With a context-free grammar, we can take as long as needed to derive something like `-(id+id)` from the start symbol `E`:

```
E => -E => -(E) => -(E + E) => -(id + E) => -(id + id)
```

### Leftmost derivations

We always choose the leftmost nonterminal in each sentence. The example above is leftmost since we are always replacing the leftmost symbol (nonterminal).

### Rightmost derivations

We always choose the rightmost nonterminal. They are also called 'canonical derivations' as they are often used for bottom up parsers.

## Parse Trees

A graphical view of a 'derivation'. It filters out the order which productions are applied to replace nonterminals.

- each internal node represents the application of a production to a sequence of chars we want to validate
- an interior node labeled `A` is a nonterminal
- nodes that are children of `A` are labeled from left -> right by symbols of the body of the production of `A`
- for any derivation `alpha_1 => alpha_2 => ... alpha_n` we can construct parse tree to yield each `alpha_i`

![](/assets/img/rei/parse_tree2.png)

- note how the 2nd and 3rd levels branch into 3 subnodes
- KEY: a non-ambiguous grammar produces only one parse tree for a valid sentence

### Ambiguous Grammars

Produces more than one leftmost or more than one rightmost derivation for the same sentence.

## Syntax Directed Translation

For context free grammars. Good for type checking and IR code generation.

```
E -> E1 + T

E = E1 || T || +
```

Basically attach attributes to the grammar symbols representing the construct. Then specify the values of attributes by associating semantic rules with grammar productions.

The above production has two non terminals E and T. E1 means the occurence of E in the production instead of the head. E and T have a string valued attribute code. And the semantic rule concat E1 code, T code and the `+` char to form E.

But it may be inefficient to implement the translation directly by manipulating strings. So we have a syntax translation scheme to embed program fragements. These are called 'semantic actions':

```
E -> E1 + T {print '+'}
```

### Syntax Directed Definition

A CFG + attributes + rules. Attributes are associated with grammar symbols. Rules are associated with productions.

If X is a symbol and a is one of its attributes, then we write X.a to denote the value of a at a particular parse tree node X. If nodes of the parse tree are implemented by records or objects. Then attributes of X can be implemented by data fields in the records that represent the nodes for X.

Attributes can be any: number, type, table reference, string. And strings may be long sequences of code like compiler IR. There are 2 types of attributes for nonterminals (expanding rules):

1. Synthesised attribute. For a nonterminal A at a parse tree node N. Defined by a semantic rule (as opposed to a syntactical rule). A must be at the head. A synthesised attribute at node Nis defined only in terms of attribute values at the children of N and at N itself
2. Inherited attribute. For a nonterminal B at a parse tree node N. Defined only in terms of attribute values at N's parent, N itself and N's siblings.

So basically either your children or your parent and siblings.

Also note for Inherited attributes. No additional translations are enabled if we allow an inherited attribute B.c at a node N to be defined in terms of attribute values at the children of N. As well as N itself, its parent and its siblings. We simulate such rule by creating additional attributes for B: B.c1, B.c2 ... These are synthesised attributes that copy the needed attributes of the children of the node B. Then compute B.c as an inherited attribute using those additional attributes. But they are rarely needed in practice.

We dont allow an inherited attribute at node N to be defined in terms of attribute values at the children of node N. But we do allow a synthesized attribute at N to be defined in terms of inherited attribute values at node N itself.

Terminals can have synthesised attributes. But not inherited attributes. Attributes for terminals have lexical values that are supplied by the lexical analyzer. There are no semantic rules in the SDD itself for computing the value of an attribute for a terminal.

An SDD usually has a bunch of production : semantic rule pairs. Each production has a nonterminal. Which has a synthesised attribute `val`. Something like a `digit` has a synthesised attribute `lexval`. So a terminal in this case has a synth attr.

When we say `L -> E n` we set `L.val` to `E.val`. T his is the numerical value for the entire expression.

If we have an SDD with only synthesised attributes, we call it an S-attributed. The above SDD is S-attr. In an S-attr SDD, each rule computes an attribute for the nonterminal at the head of a production from attributes taken from the body of the production.

Sometimes there are side effects with SDDs. Like printing the result computed or interacting with a symbol table.

Once the order of evaluation of attributes is decided. We allow semantic rules to compute arbitrary functions. Possibly involving side effects.

We can implement an S-attr SDD with an LR parser. E.g. through a yacc program. We could print E.val as a side effect instead of defining the attribute L.val.

An SDD without side effects is sometimes called an attribute grammar. The rules in an attribute grammar define the value of an attribute purely in terms of the values of other attributes and constants.

So how to evaluate an SDD at the nodes of a parse tree? A translation specified by an SDD builds a parse tree. But we do not technically need the translator.

So the rules of an SDD are applied by first constructing a parse tree then using the rules to evaluate all the attributes at each node of the tree. A parse te tree showing the values of its attributes is called an annotated parse tree.

We first evaluate all `val` attrs at all the children of a node. Before we can evaluate the `val` at the node itself. With s-attr, we eval attr in any bottom up order. Just like a postorder traversal of the tree.

Theres no guarentee that there is even one order in which to eval attributes at nodes. For nonterms A, B with synth and inherited attr A.s and B.i. Along with SDD:

```
A -> B  where  A.s = B.i; B.i = A.s + 1
```

These rules are circular. Its impossible to eval either A.s at node N or B.i at the child of N without first evaluating the other. Its computationally difficult to determine whether or not there exist any circularities in any of the parse trees that a given SDD could have to translate. But there are useful subclasses of SDDs that can guarentee that an order of evaluation exists (so that we can eventually find one that does work).

Consider an input string `3 * 5 + 4 n` constructed with an earlier grammar. The values of `lexval` are assumed to be supplied by the lexical analyzer. Each node for the nonterminal has attribute `val` computed in a bottom up order. And there are resulting values for each node. Inherited attributes are useful when the structure of a parse tree does not match the abstract syntax of the source code. If you have a grammar designed for parsing rather than translation (understanding the meaning vs transforming the input into something else with the same meaning).

The semantic rules are based on the idea that the left operand of the operator `*` is inherited. So the head T' of the production T' -> *FT1' inherits the left operand of* in the production body. Given a term x *y* z, the root of the subtree of *y* z inherits x. (the subtree attributes inherits its parent attribute).

Then the root of the subtree for *z inherits the value of x*y and so on. If there are more factors in the term. Once all the factors have been accumulated the result is passed back up the tree using synthesised attributes.

If we have an annotated parse tree `3 * 5`. The leftmost leaf is a digit with lexval = 3. Its parent node is the production F -> digit. So that node has attribute F.val which is assigned to digit.lexval. Now we need a T to do something else.

Note the next node above F must be a T. And the other child of the root is a T with an inherited value = 3. Which is a sibling node of T'. T' has children.

### Evaluation Order for SDD

We use dependency graphs to determine an evaluation order for the attribute instances for a given parse tree. While an annotated parse tree shows the alues of attributes. A dep graph helps us determine how those values can be computed.

Lets define two important classes of SDDs: S-attr and L-attr. L-attr are more general. The translations specified by these two classes fit well with the parsing methods prev. And most translations encountered in practice can be written to conform to requirements of at least one of these classes.

A dep graph depicts the flow of info among the attribute instances in a particular parse tree. An edge from one attr instance to another means that the value of the first is needed to compute the second. The rules:

1. For each parse tree node X, the dep graph has a node for each attr associated with X
2. Say a semantic rule associated with production `p` defines the value of synth attr A.b in terms of X.c. Then the dep graph has an edge from X.c -> A.b. At every node N labeled A where `p` is applied, an edge to attribute b at N. From attribute `c` at the child of N corresponding to the instance of the symbol X in the body of the production.
3. Say a semantic rule associated with production `p` defines the value of inherited attr B.c in terms of X.a. Then we have an edge X.a -> B.c. For each node N labeled B that corresponds to an occurence of this B in the body of the production `p`. Create an edge to attr `c` at N from the attr a and node M. Which corresponds to this occurence of X. Note M could either be the parent or sibling of N.

If we have E -> E1 + T. With rule E.val = E1.val + T.val. At every node labeled E. `val` of E is synth'd using the value at the two children E and T.

The parse tree edges are dotted lines. And the solid lines are the dependency graph edges.

If the dep graph has an edge from M to N. The attr of M must be eval'd before N. Then the only ordering is N1...Nk where i < j for an edge from Ni to Nj. This embeds a digraph into a linear order (based on ascending). And is called a topological sort of the graph (nodes).

If a cycle exists in the graph. Then there is no topological sorts. So there is no way to eval the SDD on this parse tree. If no cycles, then at least one topological sort.

S-attr definitions. There are two classes. The first:

An SDD is S-attr if every attr is synthesised.

When an SDD is S-attr. We eval its attr in any bottom up order.

## Applications of SDT

Constructing syntax trees! (ASTs).

ASTs are great for IR. So our SDD should turn an input string into a syntax tree. To complete the translation to IR the compiler may walk the syntax tree using another set of rules. That are in effect.

An SDD on the syntax tree rather than the actual parse tree generated. Consider two SDDs for constructing syntax trees for expressions. First, is an S-attr definition. Suitable for use during bottom up parsing. Second, L-attr, suitable during top down parsing.

### Constructing Syntax Trees

So we have a digraph and an actual parse tree. Then we have an actual SDD'd syntax tree.

Each node in a syntax tree corresponds to a construct. The children of the node represent the components of the construct. So an expr construct `E1 + E2` turns into a node with `+` with two child nodes `E1`, `E2`.

Implement the nodes of the syntax tree by objects with a suitable number of fields. Each obj has has an `op` field that labels the node. The addition fields of an object:

- If a leaf node, the additional field holds the lexical value of the leaf (token label: attr pointer). A constructor function `Leaf(op, val)` creates a leaf object. Otherwise if nodes are viewed as records then `Leaf` returns a pointer to a new record for a leaf
- If an interior node. There are as many additional fields as its number of children. A constructor function `Node(op, c1, c2 ... ck)` creates an object with a field `op` and `k` additional fields for the `k` children

### E.g. an S-attr SDD

Say we have:

- E -> E1 + T (Rule: E.node = new Node(+, E1.node, T.node))
- E -> E1 - T (Rule: E.node = new Node(-, E1.node, T.node))
- E -> T (Rule: E.node = T.node)
- T -> E (Rule: T.node = E.node)
- T- > id (Rule: T.node = new Leaf(id, id.entry))
- T -> num (Rule: T.node = new Leaf(num, num.val))

We can eval the rules with a postorder traversal of the parse tree. Or prob postorder. But no preorder traversal.

If we have an expr `a - 4 + c`. Then depending on our parse order, `+` has higher prio than `-`. Then we create a hierarchy  `+ -> (4, c)`. With `- -> (+, a)`. We can then plug that hierarchy into the part of the tree.

### E.g. an L- attr SDD

With L-attr, we cant have left recursion. We should also left factor when possible.

- E -> T E' (Rule: E.node = E'.syn && E'.inh = T.node)
- E' -> T E1' (Rule: new Node(+, E'.inh, T.node) && E'.syn = E1'.syn)
- E' -> -T E1' (Rule: new Node(-, E'.inh, T.node) && E'.syn = E1'.syn)
- E' -> eps (Rule: E'.syn = E'.inh)
- T -> (E) (Rule: T.node = E.node)
- T -> id (Rule: new Leaf(id, id.entry))
- T -> num (Rule: new Leaf(num, num.val))

With top down parsing. We start at the "root". So for `a - 4 + c`, we see `a` and `4`. Which need to be parsed as T -> num.

Steps:

1. `a` -> `Leaf(id, a)`
2. `-` -> skip, push onto stack
3. `4` -> `Leaf(num, 4)`
4. `+` -> pop `-` from stack. Push `+` onto stack
5. `-` -> Node(-, p1, p2)
6. `c` -> `Leaf(id, c)`
7. `EOF` -> pop `+` from stack.
8. `+` -> Node(+, p3, p4)
9. Nothing left in stack + EOF

## SDT Schemes

SDT schemes are a complementary notation to syntax directed definitions. All SDDs should be implementable with SDT schemes.

A SDT is a contex free grammar. With program fragments embedded within production bodies. The program fragments are called 'semantic actions'. They can appear at any position within a production body. By convention, place curly braces around actions. If braces are needed as grammar symbols, then we quote them.

Any SDT can be implemented first by building a parse tree. Then performing the actions in a left-to-right depth first order.

If we have an LR-parsable grammar with an S-attr SDD. Or an LL parsable grammar with an L-attr SDD. We can readily convert them into SDTs.

We simply introduce a marker nonterminal for each embedded action. Where `M -> eps`. If the grammar with marker non terminals can be parsed by a given method. Then the SDT can be implemented during parsing.

### Postfix Translation Schemes

By far the simplest SDD implementation occurs when we can parse the grammar bottom up and the SDD is S-attr. Here we can construct an SDT where ach eaction is placed at the end of the production. And executed along with the reduction of the body to the head of that production.

An SDT with all actions at the right ends of the production bodies is a 'postfix' SDT.

We use a parser stack. The attributes of each grammar symbol can be put on the stack in a place where they can be found during reduction. We place the attrs along with the grammar symbols in records on the stack itself.

If the parse is top down. We perform the action `a` just before we attempt to expand Y (if nonterminal). For B -> X {a} Y. Or check for Y on the input if Y is a terminal.

## Implement L-attr SDDs

We do translation by traversing a parse tree. Either:

- Build the parse tree and annotate -> good for any noncircular SDD
- Build the parse tree. Add actions. Execute actions in preorder. Turn an L-attr SDD into an SDT -> good for any L-attr definition

Then to actually tanslate durring parsing. We:

- Use a recursive descent parser. With one function for each nonterminal. The function for nonterminal A receives the inherited attributes of A as arguments. And returns the synth'd attr of A