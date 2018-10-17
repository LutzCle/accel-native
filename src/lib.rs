/*
 * Copyright 2018 German Research Center for Artificial Intelligence (DFKI)
 * Author: Clemens Lutz <clemens.lutz@dfki.de>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[macro_export]
macro_rules! cuda {
    ($kernel_name:ident<<[$module:expr, $grid:expr, $block:expr]>>($($arg:expr),*)) => {
        {
            let module: &self::accel::module::Module = &$module;
            let mut kernel = module.get_kernel(stringify!($kernel_name));
            let mut args = [
                $(self::accel::kernel::void_cast(&$arg),)*
            ];
            match &mut kernel {
                Ok(k) => unsafe {
                    k.launch(
                        args.as_mut_ptr(),
                        $grid,
                        $block,
                        )
                },
                Err(e) => Err(*e),
            }
        }
    };
    ([$module:expr]::$kernel_name:ident<<[$grid:expr, $block:expr]>>($($arg:expr),*)) => {

        {
            let module = $module;
            cuda!(module::$kernel_name<<[$grid, $block]>>($($arg:expr,)*))
        }

    };
}
