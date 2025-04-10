# **med - Mini-editor**  

**A line-based text editor inspired by `ed`, initially made for embedded systems (MicroPython, Raspberry Pi Pico),**\
**but runs on any platform.**  


[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)  
![Python](https://img.shields.io/badge/Python-3.x%20%7C%20MicroPython-green)  
![Future: Rust+LuaJIT](https://img.shields.io/badge/Future-Rust+LuaJIT-orange)  

---

## **📌 Overview**  
```
Mini-editor (type 'h' for help)
                   __        
.--------.-----.--|  |══     
|        |  -__|  _  |════   
|__|__|__|_____|_____|═══════

* 
```
**med** (*Mini-editor*) is a simple text editor designed for **embedded systems** (like Raspberry Pi Pico) where editing files directly in the MicroPython REPL is cumbersome.  

🔹 **Minimalism** – works in raw terminal environments.  
🔹 **Embedded-friendly** – runs on MicroPython with minimal dependencies.  
🔹 **ed-like** – command editing.  
🔹 **Lightweight** – No GUI, just pure terminal interaction.  

---

## **🛠 Usage**  
```bash
med                # Start with empty buffer
```  
---

### **Commands**  
| Command      | Action                               |  
|--------------|--------------------------------------|  
| ld           | list directories in the current path |  
| cd [path]    | change the current working directory |  
| a            | append text                          |  
| i[bfre]      | insert text                          |  
| p[:from:,to] | print buffer                         |  
| s [expr]     | search in buffer                     |
| d[from:,to]  | delete from buffer                   |
| w [file]     | write to file                        |
| r [file]     | read from file                       |
| q            | quit                                 |
| h            | help                                 |

---


**💡 Why Med?**  
Because editing code in `picocom`/`minicom` over serial should be less painful!  
