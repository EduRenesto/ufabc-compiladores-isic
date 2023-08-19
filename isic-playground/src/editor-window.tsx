import React, { useState } from "react";

import "ace-builds/src-noconflict/ace";
import "ace-builds/src-noconflict/mode-java";
import "ace-builds/src-noconflict/theme-terminal";

import AceEditor from "react-ace";
import { Button, Select, ButtonsWrapper } from "./editor-window.style";
import { Window } from "./window";

const sampleText = `programa
    declare a: int.
    declare b: int.

    a := 10.
    leia(b).

    declare c: int.
    c := a * b.

    declare d: float.

    escreva(c).
fimprog.`;

const HELLO_WORLD = `programa
    escreva("Hello world").
fimprog.`;

const IO = `programa
    declare a: int.
    declare b: int.

    leia(a).
    leia(b).

    declare c: int.
    c := a * b.

    escreva(c).
fimprog.
`;

const FIBONACCI = `programa
    declare n: int.
    leia(n).

    declare i: int.
    i := 0.

    declare x1: int.
    x1 := 0.

    declare x2: int.
    x2 := 1.

    declare x: int.
    enquanto (i < n) {
        escreva(x1).

        x := x1 + x2.

        x1 := x2.
        x2 := x.

        i := i + 1.
    }
fimprog.
`;

const COLLATZ = `TODO`;

const EXAMPLES = new Map([
    ["hello-world", HELLO_WORLD],
    ["io", IO],
    ["fibonacci", FIBONACCI],
    ["collatz", COLLATZ],
]);

export type EditorProps = {
    onCompile: (text: string) => void;
    onEvaluate: (text: string) => void;
};

export const EditorWindow: React.FC<EditorProps> = (props: EditorProps) => {
    const [example, setExample] = useState<string>("hello-world");
    const [text, setText] = useState(EXAMPLES.get("hello-world"));

    return (
        <Window title="playground.isi - Editor">
            <p>Examples</p>
            <ButtonsWrapper>
                <Select onChange={(e) => setExample(e.target.value)} defaultValue="hello-world">
                    <option value="hello-world">Hello world</option>
                    <option value="io">I/O</option>
                    <option value="fibonacci">Fibonacci</option>
                    <option value="conditional">Conditionals</option>
                </Select>
                <Button onClick={() => setText(EXAMPLES.get(example))}>Load example</Button>
            </ButtonsWrapper>

            <AceEditor
                onChange={setText}
                value={text}
            />

            <ButtonsWrapper>
                <Button onClick={() => props.onCompile(text)}>Compile</Button>
                <Button onClick={() => props.onEvaluate(text)}>Run interpreter</Button>
            </ButtonsWrapper>
        </Window>
    )
}
