# KiraCode Documentation: Abstract Syntax Tree

An "_Abstract Syntax Tree_" is a branching flowchart-like tree of tokens for a compiler to easier compile.

> ###### TODO:
>
> Improve description details!

---

## Outline

- [Introduction](./introduction.md)
- [Tokens](./tokens.md)

---

## ABS: LaTeX Visualisation

$$
\begin{align}
  [\text{prog}]
  &\to
  \begin{cases}
    [\text{expr}]
    &\to
    \begin{cases}
      [\text{ltrl}]
      \\\\
      [\text{stmt}]
    \end{cases}
  \end{cases}
  \\
\end{align}
$$
