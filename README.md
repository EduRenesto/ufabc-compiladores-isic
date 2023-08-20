# isic

## TL;DR

- Autor: Eduardo Renesto Estanquiere - 11201810086
- Link do video: [https://youtu.be/pUXbtrRK3GU](https://youtu.be/pUXbtrRK3GU)
- Link do playground: [https://isic-playground.netlify.app/](https://isic-playground.netlify.app/)

## Intro

Este repositório contém o código completo do *isic*, minha implementação do
projeto da disciplina Compiladores, ofertada na UFABC 2023.2 pelo brabíssimo
Professor Isidro.

O código está separado em vários subprojetos:

- `isic-front`: gramática, AST e parser da IsiLang
- `isic-middle`: validadores da AST - type checker e usage checker
- `isic-back`: emissor de código C
- `isic-interpreter`: interpretador/runtime para a IsiLang
- `isic-cli`: executável principal do projeto, implementa a CLI do isic e faz a
  ligação entre os subprojetos
- `isic-playground-glue`: código cola que exporta os pontos de entrada da
  compilação e interpretação em WebAssembly
- `isic-playground`: app em TypeScript/React que usa o bundle WASM do compilador
  e apresenta um editor de texto para brincar com a linguagem
  
## Dialeto da IsiLang

Como esperado do projeto, algumas mudanças foram feitas na gramática da IsiLang.

### Declaração de Variáveis

As declarações de variáveis *devem* incluir o tipo da variável. Declarações não
tipadas não são permitidas.

``` isilang
declare x: int.
declare y: float.
declare s: string.
```

**OBS:** `strings` só funcionam corretamente no interpretador.

### Operadores lógicos

Foram adicionados os operadores `&&`, `||` e `!`.

``` isilang
se (x < y && y < z || !(a == b)) entao {
    ...
}
```

### Operadores aritméticos

Além dos operadores padrão, foi adicionado o operador `%` (resto da divisão).

``` isilang
se (x % 2 == 0) entao {
    escreva("x par").
} senao {
    escreva("x impar").
}
```

### Exemplos

Vide os exemplos na pasta `examples/`, e os disponíveis no playground.
  
## Compilando e executando
  
### Requisitos

O core do projeto foi escrito em Rust, e portanto para rodá-lo será necessária
uma toolchain Rust. A maneira mais simples de obter uma é usando o
[rustup.rs](https://rustup.rs).

Para o playground, é necessário `node.js` (utilizei a versão 18, mas pode
funcionar com versões mais antigas) e o `yarn`. Para compilar o bundle WASM, é
necessária a ferramenta
[wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) e o target
`wasm32-unknown-unknown` instalado. A maneira mais simples é rodar o seguinte
comando com o `rustup`:

``` sh
$ rustup target add wasm32-unknown-unknown
```

### isic-cli

Instalado o toolchain, basta entrar na pasta raiz do projeto e rodar o seguinte
comando:

``` sh
$ cargo run -- <cli args>
```

### isic-playground

Instaladas as dependências, siga os seguintes passos, a partir da raiz do projeto:

``` sh
$ cd isic-playground-glue
$ wasm-pack build
$ cd ../isic-playground
$ yarn install
```

Para subir o servidor de desenvolvimento, basta rodar um `$ yarn dev`. Caso
queria compilar a distribuição e bundle JavaScript, siga os mesmos passos e rode
`$ yarn build`. A saída estará na pasta `isic-playground/dist`.

Para referência, veja o arquivo `.github/workflows/ci.yaml`.

**Nota.** A *developer experience* da integração Rust <-> WASM <-> JavaScript
não é a mais fluida. Portanto, a cada mudança no código Rust, é recomendado
apagar a `node_modules` e o `yarn.lock` do projeto `isic-playground` e
reexecutar os passos de compilação do playground.
