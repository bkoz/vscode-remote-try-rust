/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/

fn add(x: u32, y: u32) -> u32 {
    return x + y;
}
 fn main() {
    let name = "VS Code Remote - Containers";
    println!("Hello, {} {}!", name, add(2, 3));
}