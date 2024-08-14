import { Table, TableBody, TableCell, TableHead, TableRow } from "@mui/material"
import { CpuStep, StepLog } from "../log"
import { useEffect, useRef } from "react";

interface StepLogTableProps
{
    stepLog: StepLog | undefined
    step: number
}

export default function StepLogTable(props: Readonly<StepLogTableProps>)
{
    const cpuSteps: CpuStep[] | undefined = props.stepLog?.steps;
    const activeRowRef = useRef<HTMLTableRowElement>(null);

    useEffect(() =>
    {
        if (activeRowRef.current)
        {
            activeRowRef.current.scrollIntoView({ behavior: "smooth", block: "center" });
        }
    }, [props.step]);

    return (
        <Table>
            <TableHead>
                <TableRow>
                    <TableCell>Step</TableCell>
                    <TableCell align="right">Fetched instruction</TableCell>
                    <TableCell>Decoded instruction</TableCell>
                    <TableCell>RAM writes</TableCell>
                </TableRow>
            </TableHead>
            <TableBody>
                {
                    cpuSteps?.map((cpuStep: CpuStep, index: number) =>
                    {
                        const isActive = props.step == cpuStep.step + 1;

                        return (
                            <TableRow key={index.toString()} sx={{
                                backgroundColor: isActive ? "lightblue" : "white"
                            }} ref={isActive ? activeRowRef : null}>
                                <TableCell>{cpuStep.step + 1}</TableCell>
                                <TableCell align="right">0x{cpuStep.fetched_instruction.toString(16).padStart(8, "0")}</TableCell>
                                <TableCell>{JSON.stringify(cpuStep.decoded_instruction)}</TableCell>
                                <TableCell>{JSON.stringify(cpuStep.ram_writes)}</TableCell>
                            </TableRow>
                        )
                    })
                }
            </TableBody>
        </Table>
    )
}
