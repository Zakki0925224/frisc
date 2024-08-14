import { Table, TableBody, TableCell, TableHead, TableRow } from "@mui/material";
import { CpuStep, StepLog } from "../log"
import { useEffect, useRef, useState } from "react";

function numArrayTou32Array(numArray: number[]): Uint32Array
{
    const u8Array = new Uint8Array(numArray);
    const u32Array = new Uint32Array(u8Array.buffer);
    return u32Array;
}

interface RamTableProps
{
    stepLog: StepLog | undefined
    step: number
}

export default function RamTable(props: Readonly<RamTableProps>)
{
    const [u32Ram, setU32Ram] = useState<Uint32Array | undefined>(undefined);
    const [rows, setRows] = useState<any[]>([]);
    const [activeRamAddress, setActiveRamAddress] = useState<number[]>([]);
    const activeRowRef = useRef<HTMLTableRowElement>(null);

    useEffect(() =>
    {
        if (activeRowRef.current)
        {
            activeRowRef.current.scrollIntoView({ behavior: "smooth", block: "center" });
        }
    }, [props.step]);

    useEffect(() =>
    {
        if (props.stepLog === undefined)
        {
            return;
        }

        const ram = [...props.stepLog.init_ram];
        const activeRamAddress: number[] = [];

        for (let i = 0; i < props.step; i++)
        {
            const cpuStep: CpuStep | undefined = props.stepLog.steps[i];
            cpuStep?.ram_writes.forEach(v =>
            {
                ram[v.addr] = v.value;

                if (props.step === cpuStep.step + 1)
                {
                    activeRamAddress.push(v.addr);
                }
            });
        }

        setU32Ram(numArrayTou32Array(ram));
        setActiveRamAddress(activeRamAddress);
    }, [props.step, props.stepLog]);

    useEffect(() =>
    {
        const rows = [];

        for (let i = 0; i < (u32Ram !== undefined ? u32Ram.length : 0);)
        {
            let start = i;
            const value = u32Ram![i];
            while (i < u32Ram!.length && u32Ram![i] === value)
            {
                i++;
            }

            start *= 4;
            const end = (i - 1) * 4 + 3;
            rows.push({
                isActive: activeRamAddress.find(addr => addr >= start && addr <= end) !== undefined,
                start,
                end,
                value,
            });
        }

        setRows(rows);
    }, [u32Ram, activeRamAddress]);

    return (
        <Table>
            <TableHead>
                <TableRow>
                    <TableCell align="right">Address</TableCell>
                    <TableCell align="right">Value (u32)</TableCell>
                </TableRow>
            </TableHead>
            <TableBody>
                {rows.map((row, index) => (
                    <TableRow key={index.toString()} sx={{ backgroundColor: row.isActive ? "lightyellow" : "white" }} ref={row.isActive ? activeRowRef : null}>
                        <TableCell align="right">
                            0x{row.start.toString(16).padStart(8, "0")} - 0x{row.end.toString(16).padStart(8, "0")}
                        </TableCell>
                        <TableCell align="right">0x{row.value.toString(16).padStart(8, "0")}</TableCell>
                    </TableRow>
                ))}
            </TableBody>
        </Table>
    );

}
