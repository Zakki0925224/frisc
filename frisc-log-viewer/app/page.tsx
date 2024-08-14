"use client";

import { Button, Card, CardContent, Grid, Slider, Typography } from "@mui/material";
import { MuiFileInput } from "mui-file-input";
import { useEffect, useState } from "react";
import RegisterTable from "./components/register_table";
import { StepLog } from "./log";
import StepLogTable from "./components/step_log_table";
import RamTable from "./components/ram_table";

export default function Home()
{
    const [file, setFile] = useState<File | null>(null);
    const [stepLog, setStepLog] = useState<StepLog | undefined>(undefined);
    const [step, setStep] = useState<number>(0);
    const [isPlaying, setIsPlaying] = useState<boolean>(false);

    useEffect(() =>
    {
        let interval: NodeJS.Timeout | null = null;
        if (isPlaying && stepLog)
        {
            interval = setInterval(() =>
            {
                setStep((prevStep) =>
                {
                    const newStep = prevStep + 1;
                    if (newStep >= stepLog.steps.length)
                    {
                        clearInterval(interval!);
                        setIsPlaying(false);
                        return prevStep;
                    }
                    return newStep;
                });
            }, 1000); // 1s
        } else if (!isPlaying && interval)
        {
            clearInterval(interval);
        }

        return () =>
        {
            if (interval)
            {
                clearInterval(interval);
            }
        };
    }, [isPlaying, stepLog]);

    const onChangeFileInput = (file: File | null) =>
    {
        setFile(file);

        // parse json
        if (file)
        {
            const reader = new FileReader();
            reader.readAsText(file);
            reader.onload = () =>
            {
                const data = reader.result as string;
                try
                {
                    const stepLog = JSON.parse(data);
                    setStepLog(stepLog);
                }
                catch (e)
                {
                    console.error(e)
                }
            }
            reader.onerror = () =>
            {
                console.error(reader.error);
            }
        }
    };

    return (
        <main>
            <h1>frisc-log-viewer</h1>
            <p>Visualize frisc(RV32I) emulation log.</p>
            <Grid container spacing={2}>
                <Grid item xs={12}>
                    <Card>
                        <CardContent>
                            <MuiFileInput label="Log file" value={file} onChange={onChangeFileInput} style={{ width: "100%" }} />
                        </CardContent>
                    </Card>
                </Grid>
                <Grid item xs={12}>
                    <Card>
                        <CardContent>
                            <Button variant="contained" onClick={() => setIsPlaying((prev) => !prev)}>
                                {isPlaying ? "Stop" : "Play"}
                            </Button>
                            <Slider
                                aria-label="Steps"
                                defaultValue={0}
                                valueLabelDisplay="auto"
                                step={1}
                                min={0}
                                max={stepLog !== undefined ? stepLog.steps.length : 0}
                                marks
                                value={step}
                                onChange={(_, v) =>
                                {
                                    if (typeof v === "number")
                                    {
                                        setStep(v);
                                    }
                                }}
                            />
                        </CardContent>
                    </Card>
                </Grid>
                <Grid item xs={4}>
                    <Card sx={{ maxHeight: 600, overflow: "auto" }}>
                        <CardContent>
                            <Typography variant="h5" component="div">
                                Registers
                            </Typography>
                        </CardContent>
                        <RegisterTable stepLog={stepLog} step={step} />
                    </Card>
                </Grid>
                <Grid item xs={4}>
                    <Card sx={{ maxHeight: 600, overflow: "auto" }}>
                        <CardContent>
                            <Typography variant="h5" component="div">
                                Step log
                            </Typography>
                        </CardContent>
                        <StepLogTable stepLog={stepLog} step={step} />
                    </Card>
                </Grid>
                <Grid item xs={4}>
                    <Card sx={{ maxHeight: 600, overflow: "auto" }}>
                        <CardContent>
                            <Typography variant="h5" component="div">
                                RAM
                            </Typography>
                        </CardContent>
                        <RamTable stepLog={stepLog} step={step} />
                    </Card>
                </Grid>
            </Grid>
        </main>
    );
}
