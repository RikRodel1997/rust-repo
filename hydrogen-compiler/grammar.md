$$
\begin{align}

    [\text{prog}] &\to [\text{stmt}]^*
    \\
    [\text{stmt}] &\to
    \begin{cases}
        exit([\text{expr}]);
        \\
        let\space\text{ident} = [\text{expr}]
    \end{cases}
    \\
    [\text{expr}] &\to
    \begin{cases}
        \text{integer\_literal}
        \\
        \text{identifier}
                \\
        \text{binary\_expr}
    \end{cases}
    \\
    [\text{binary\_expr}] &\to
    \begin{cases}
        [\text{expr}] * [\text{expr}] & \text{prec} = 1
        \\
        [\text{expr}] + [\text{expr}] & \text{prec} = 0
    \end{cases}

\end{align}
$$
