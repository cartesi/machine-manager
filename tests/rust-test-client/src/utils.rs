// Copyright 2021 Cartesi Pte. Ltd.
//
// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use std::{
    io::{BufRead, BufReader},
    process::ChildStderr,
    sync::mpsc::{Receiver, Sender},
    thread::{sleep, spawn},
    time::Duration,
};

pub fn wait_process_output(
    receiver: &Receiver<String>,
    output: Vec<(String, u16)>,
) -> Result<(), String> {
    let mut expect = output;
    let mut num_of_probes = 0;
    loop {
        if num_of_probes > 10 {
            return Err("Unable to get desired output: timeout".to_string());
        }
        match receiver.try_recv() {
            Ok(val) => {
                for i in 0..expect.len() {
                    let (key, amount) = expect[i].clone();
                    if val.contains(&key) {
                        expect[i] = (key, amount - 1);
                        if amount - 1 == 0 {
                            expect.remove(i);
                        }
                        break;
                    }
                }
                if expect.is_empty() {
                    return Ok(());
                }
            }
            Err(_) => {
                num_of_probes += 1;
                sleep(Duration::new(1, 0));
            }
        }
    }
}

pub fn start_listener(sender: Sender<String>, receiver: Receiver<()>, stream: ChildStderr) {
    spawn(move || {
        let mut f = BufReader::new(stream);
        loop {
            let mut num_of_probes = 0;
            if let Ok(()) = receiver.try_recv() {
                return;
            }
            let mut buf = String::new();
            match f.read_line(&mut buf) {
                Ok(_) => {
                    if !buf.is_empty() {
                        println!("{}", buf);
                        num_of_probes = 0;
                    } else {
                        num_of_probes += 1;
                    }
                    if num_of_probes > 10 {
                        return;
                    }
                    if let Err(e) = sender.send(buf) {
                        eprintln!("Message send failed: {}", e);
                    }
                    continue;
                }
                Err(_) => break,
            }
        }
    });
}
