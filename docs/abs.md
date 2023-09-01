# KiraCode Documentation: Abstract Syntax Tree

An "_Abstract Syntax Tree_" is a branching flowchart-like tree of tokens for a compiler to easier compile.

> ## ! TODO
>
> Improve description details!

---

## Outline

- [Introduction](./introduction.md)
- [Tokens](./tokens.md)
- [Usage](./usage.md)

---

## ABS: LaTeX Visualisation

$$
\begin{align}
  [\text{expr}]
  &\to
  \begin{cases}
    [\text{ltrl}]
    &\to
    \begin{cases}
      [\text{num}]
      \\\\
      [\text{char}]
      \\\\
      [\text{paren}]
    \end{cases}
    \\\\
    [\text{var}]
    \\\\
    [\text{fun}]
    \\\\
    [\text{block}]
    \\\\
    [\text{const}]
    \\\\
    [\text{alias}]
    \\\\
    [\text{struc}]
    \\\\
    [\text{enum}]
  \end{cases}
  \\\\
  [\text{stmt}]
    &\to
    \begin{cases}
      [\text{let [ident]: [type] = [expr];}]
      &\to
      [\text{stmt.let}]
      \\\\
      [\text{fun [ident] ([ident]: [type] = [expr]) [block]}]
      &\to
      [\text{stmt.fun}]
      \\\\
      [\text{const [ident]: [type] = [expr];}]
      &\to
      [\text{stmt.const}]
      \\\\
      [\text{alias [ident] = [ident];}]
      &\to
      [\text{stmt.alias}]
      \\\\
      [\text{struct [ident] [body.struc]}]
      &\to
      [\text{stmt.struc}]
      \\\\
      [\text{enum [ident] [body.enum]}]
      &\to
      [\text{stmt.enum}]
    \end{cases}
  \\\\
  [\text{oper}]
  &\to
  \begin{cases}
    [\text{open}]
    &\to
    \begin{cases}
      [\text{brace}] &\leftarrow ['\{']
      \\\\
      [\text{brack}] &\leftarrow ['[']
      \\\\
      [\text{paren}] &\leftarrow ['(']
    \end{cases}
    \\\\
    [\text{close}]
    &\to
    \begin{cases}
      [\text{brace}] &\leftarrow ['\}']
      \\\\
      [\text{brack}] &\leftarrow [']']
      \\\\
      [\text{paren}] &\leftarrow [')']
    \end{cases}
    \\\\
    [\text{mths}]
    &\to
    \begin{cases}
      [\text{div}] &\leftarrow ['/']
      \\\\
      [\text{mul}] &\leftarrow ['*']
      \\\\
      [\text{add}] &\leftarrow ['+']
      \\\\
      [\text{sub}] &\leftarrow ['-']
    \end{cases}
    \\\\
    [\text{asgn}]
    &\to
    \begin{cases}
      [\text{eq}]    &\leftarrow ['=']
      \\\\
      [\text{diveq}] &\leftarrow ['/=']
      \\\\
      [\text{muleq}] &\leftarrow ['*=']
      \\\\
      [\text{addeq}] &\leftarrow ['+=']
      \\\\
      [\text{subeq}] &\leftarrow ['-=']
    \end{cases}
  \end{cases}
  \\
\end{align}
$$
