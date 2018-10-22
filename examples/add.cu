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

#include <cstdint>

extern "C"
__global__ void add(int32_t a, int32_t b, int64_t* c) {
    *c = a + b;
}

extern "C"
__global__ void vector_add(uint64_t len, int64_t *a, int64_t const* b) {

    for (uint64_t i = 0; i < len; ++i) {
        a[i] = a[i] + b[i];
    }

}
