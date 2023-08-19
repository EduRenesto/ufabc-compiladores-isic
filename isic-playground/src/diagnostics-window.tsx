import React from "react";
import { Window } from "./window";

export type DiagnosticsProps = {
    errors: string[],
    warns: string[],
};

export const DiagnosticsWindow: React.FC<DiagnosticsProps> = (props: DiagnosticsProps) => {
    return (
        <Window title="Diagnostics" disabled={props.errors.length === 0 && props.warns.length === 0}>
            <p>Errors</p>
            <pre>{props.errors.join("\n")}</pre>
            <p>Warnings</p>
            <pre>{props.warns.join("\n")}</pre>
        </Window>
    )
};
