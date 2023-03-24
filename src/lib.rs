/*
 * This file is part of PROJECT.
 *
 * PROJECT is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * PROJECT is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with PROJECT.  If not, see <https://www.gnu.org/licenses/>.
 */

use glob::glob;

pub struct GlobObj {
    pub iter: glob::Paths,
}

impl Iterator for GlobObj {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(path)) => {
                Some(path.to_str().expect("err").to_owned())
            }
            _ => None,
        }
    }
}

pub fn globobj(pattern: &str) -> GlobObj {
    GlobObj {
        iter: glob(pattern).expect("failed"),
    }
}

