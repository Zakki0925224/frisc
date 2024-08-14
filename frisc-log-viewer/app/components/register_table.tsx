import { Table, TableBody, TableCell, TableHead, TableRow } from "@mui/material";
import { useEffect, useRef, useState } from "react";
import { CpuState, CpuStep, StepLog } from "../log";

interface RegisterTableRowProps
{
    name: string;
    value: number;
    isHighlighted: boolean;
}

function RegisterTableRow(props: Readonly<RegisterTableRowProps>)
{
    const activeRowRef = useRef<HTMLTableRowElement>(null);

    useEffect(() =>
    {
        if (activeRowRef.current)
        {
            activeRowRef.current.scrollIntoView({ behavior: "smooth", block: "center" });
        }
    }, [props.isHighlighted]);

    return (
        <TableRow sx={{ backgroundColor: props.isHighlighted ? "lightgreen" : "white" }} ref={props.isHighlighted ? activeRowRef : null}>
            <TableCell>{props.name}</TableCell>
            <TableCell align="right">{props.value.toString(10)}</TableCell>
            <TableCell align="right">0x{props.value.toString(16).padStart(8, "0")}</TableCell>
        </TableRow>
    );
}

interface RegisterTableProps
{
    stepLog: StepLog | undefined;
    step: number;
}

export default function RegisterTable(props: Readonly<RegisterTableProps>)
{
    const prevCpuStep: CpuStep | undefined = props.stepLog?.steps[props.step - 2];
    const cpuStep: CpuStep | undefined = props.stepLog?.steps[props.step - 1];
    const cpuState: CpuState | undefined = props.step > 0 ? cpuStep?.cpu_state : props.stepLog?.init_cpu_state;

    const isRegisterUpdated = (register: keyof CpuState) =>
    {
        const prevCpuState: CpuState | undefined = prevCpuStep?.cpu_state;

        if (prevCpuState !== undefined && cpuState !== undefined)
        {
            return prevCpuState[register] !== cpuState[register];
        }

        return false;
    };

    return (
        <Table>
            <TableHead>
                <TableRow>
                    <TableCell>Register</TableCell>
                    <TableCell align="right">Decimal</TableCell>
                    <TableCell align="right">Hex</TableCell>
                </TableRow>
            </TableHead>
            <TableBody>
                <RegisterTableRow name="pc" value={cpuState?.pc ?? 0} isHighlighted={isRegisterUpdated("pc")} />
                <RegisterTableRow name="x0 (zero)" value={cpuState?.zero ?? 0} isHighlighted={isRegisterUpdated("zero")} />
                <RegisterTableRow name="x1 (ra)" value={cpuState?.ra ?? 0} isHighlighted={isRegisterUpdated("ra")} />
                <RegisterTableRow name="x2 (sp)" value={cpuState?.sp ?? 0} isHighlighted={isRegisterUpdated("sp")} />
                <RegisterTableRow name="x3 (gp)" value={cpuState?.gp ?? 0} isHighlighted={isRegisterUpdated("gp")} />
                <RegisterTableRow name="x4 (tp)" value={cpuState?.tp ?? 0} isHighlighted={isRegisterUpdated("tp")} />
                <RegisterTableRow name="x5 (t0)" value={cpuState?.t0 ?? 0} isHighlighted={isRegisterUpdated("t0")} />
                <RegisterTableRow name="x6 (t1)" value={cpuState?.t1 ?? 0} isHighlighted={isRegisterUpdated("t1")} />
                <RegisterTableRow name="x7 (t2)" value={cpuState?.t2 ?? 0} isHighlighted={isRegisterUpdated("t2")} />
                <RegisterTableRow name="x8 (s0/fp)" value={cpuState?.s0 ?? 0} isHighlighted={isRegisterUpdated("s0")} />
                <RegisterTableRow name="x9 (s1)" value={cpuState?.s1 ?? 0} isHighlighted={isRegisterUpdated("s1")} />
                <RegisterTableRow name="x10 (a0)" value={cpuState?.a0 ?? 0} isHighlighted={isRegisterUpdated("a0")} />
                <RegisterTableRow name="x11 (a1)" value={cpuState?.a1 ?? 0} isHighlighted={isRegisterUpdated("a1")} />
                <RegisterTableRow name="x12 (a2)" value={cpuState?.a2 ?? 0} isHighlighted={isRegisterUpdated("a2")} />
                <RegisterTableRow name="x13 (a3)" value={cpuState?.a3 ?? 0} isHighlighted={isRegisterUpdated("a3")} />
                <RegisterTableRow name="x14 (a4)" value={cpuState?.a4 ?? 0} isHighlighted={isRegisterUpdated("a4")} />
                <RegisterTableRow name="x15 (a5)" value={cpuState?.a5 ?? 0} isHighlighted={isRegisterUpdated("a5")} />
                <RegisterTableRow name="x16 (a6)" value={cpuState?.a6 ?? 0} isHighlighted={isRegisterUpdated("a6")} />
                <RegisterTableRow name="x17 (a7)" value={cpuState?.a7 ?? 0} isHighlighted={isRegisterUpdated("a7")} />
                <RegisterTableRow name="x18 (s2)" value={cpuState?.s2 ?? 0} isHighlighted={isRegisterUpdated("s2")} />
                <RegisterTableRow name="x19 (s3)" value={cpuState?.s3 ?? 0} isHighlighted={isRegisterUpdated("s3")} />
                <RegisterTableRow name="x20 (s4)" value={cpuState?.s4 ?? 0} isHighlighted={isRegisterUpdated("s4")} />
                <RegisterTableRow name="x21 (s5)" value={cpuState?.s5 ?? 0} isHighlighted={isRegisterUpdated("s5")} />
                <RegisterTableRow name="x22 (s6)" value={cpuState?.s6 ?? 0} isHighlighted={isRegisterUpdated("s6")} />
                <RegisterTableRow name="x23 (s7)" value={cpuState?.s7 ?? 0} isHighlighted={isRegisterUpdated("s7")} />
                <RegisterTableRow name="x24 (s8)" value={cpuState?.s8 ?? 0} isHighlighted={isRegisterUpdated("s8")} />
                <RegisterTableRow name="x25 (s9)" value={cpuState?.s9 ?? 0} isHighlighted={isRegisterUpdated("s9")} />
                <RegisterTableRow name="x26 (s10)" value={cpuState?.s10 ?? 0} isHighlighted={isRegisterUpdated("s10")} />
                <RegisterTableRow name="x27 (s11)" value={cpuState?.s11 ?? 0} isHighlighted={isRegisterUpdated("s11")} />
                <RegisterTableRow name="x28 (t3)" value={cpuState?.t3 ?? 0} isHighlighted={isRegisterUpdated("t3")} />
                <RegisterTableRow name="x29 (t4)" value={cpuState?.t4 ?? 0} isHighlighted={isRegisterUpdated("t4")} />
                <RegisterTableRow name="x30 (t5)" value={cpuState?.t5 ?? 0} isHighlighted={isRegisterUpdated("t5")} />
                <RegisterTableRow name="x31 (t6)" value={cpuState?.t6 ?? 0} isHighlighted={isRegisterUpdated("t6")} />
            </TableBody>
        </Table>
    );
}
