{
  "creator": "Yosys 0.8 (git sha1 5706e90)",
  "modules": {
    "full_adder": {
      "attributes": {
        "src": "sample.v:1"
      },
      "ports": {
        "A": {
          "direction": "input",
          "bits": [ 2, 3, 4, 5 ]
        },
        "B": {
          "direction": "input",
          "bits": [ 6, 7, 8, 9 ]
        },
        "X": {
          "direction": "output",
          "bits": [ 10, 11, 12, 13 ]
        },
        "carry": {
          "direction": "output",
          "bits": [ 14 ]
        }
      },
      "cells": {
        "$auto$simplemap.cc:85:simplemap_bitop$110": {
          "hide_name": 1,
          "type": "$_AND_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:221"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 15 ],
            "B": [ 16 ],
            "Y": [ 17 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$111": {
          "hide_name": 1,
          "type": "$_AND_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:221"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 18 ],
            "B": [ 19 ],
            "Y": [ 20 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$112": {
          "hide_name": 1,
          "type": "$_AND_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:221"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 21 ],
            "B": [ 22 ],
            "Y": [ 23 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$113": {
          "hide_name": 1,
          "type": "$_AND_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:222"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 18 ],
            "B": [ 24 ],
            "Y": [ 21 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$114": {
          "hide_name": 1,
          "type": "$_AND_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:229"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 24 ],
            "B": [ 22 ],
            "Y": [ 25 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$117": {
          "hide_name": 1,
          "type": "$_OR_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:221"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 26 ],
            "B": [ 17 ],
            "Y": [ 22 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$118": {
          "hide_name": 1,
          "type": "$_OR_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:221"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 27 ],
            "B": [ 20 ],
            "Y": [ 28 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$119": {
          "hide_name": 1,
          "type": "$_OR_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:221"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 28 ],
            "B": [ 23 ],
            "Y": [ 14 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$120": {
          "hide_name": 1,
          "type": "$_OR_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:229"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 19 ],
            "B": [ 25 ],
            "Y": [ 29 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$54": {
          "hide_name": 1,
          "type": "$_XOR_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:262"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 2 ],
            "B": [ 6 ],
            "Y": [ 10 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$55": {
          "hide_name": 1,
          "type": "$_XOR_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:262"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 3 ],
            "B": [ 7 ],
            "Y": [ 15 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$56": {
          "hide_name": 1,
          "type": "$_XOR_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:262"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 4 ],
            "B": [ 8 ],
            "Y": [ 24 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$57": {
          "hide_name": 1,
          "type": "$_XOR_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:262"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 5 ],
            "B": [ 9 ],
            "Y": [ 18 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$60": {
          "hide_name": 1,
          "type": "$_XOR_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:263"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 15 ],
            "B": [ 16 ],
            "Y": [ 11 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$61": {
          "hide_name": 1,
          "type": "$_XOR_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:263"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 24 ],
            "B": [ 22 ],
            "Y": [ 12 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$62": {
          "hide_name": 1,
          "type": "$_XOR_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:263"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 18 ],
            "B": [ 29 ],
            "Y": [ 13 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$65": {
          "hide_name": 1,
          "type": "$_AND_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 2 ],
            "B": [ 6 ],
            "Y": [ 16 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$66": {
          "hide_name": 1,
          "type": "$_AND_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 3 ],
            "B": [ 7 ],
            "Y": [ 26 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$67": {
          "hide_name": 1,
          "type": "$_AND_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 4 ],
            "B": [ 8 ],
            "Y": [ 19 ]
          }
        },
        "$auto$simplemap.cc:85:simplemap_bitop$68": {
          "hide_name": 1,
          "type": "$_AND_",
          "parameters": {
          },
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260"
          },
          "port_directions": {
            "A": "input",
            "B": "input",
            "Y": "output"
          },
          "connections": {
            "A": [ 5 ],
            "B": [ 9 ],
            "Y": [ 27 ]
          }
        }
      },
      "netnames": {
        "$techmap$add$sample.v:5$1.$auto$alumacc.cc:474:replace_alu$46.lcu.g": {
          "hide_name": 1,
          "bits": [ 16, 22, 29, 30, 31 ],
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:203"
          }
        },
        "$techmap$add$sample.v:5$1.$auto$alumacc.cc:490:replace_alu$47": {
          "hide_name": 1,
          "bits": [ 32, 15, 24, 18, 33 ],
          "attributes": {
          }
        },
        "$techmap$techmap$add$sample.v:5$1.$auto$alumacc.cc:474:replace_alu$46.$and$<techmap.v>:260$51_Y": {
          "hide_name": 1,
          "bits": [ 34, 26, 19, 27, 35 ],
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260"
          }
        },
        "$techmap$techmap$add$sample.v:5$1.$auto$alumacc.cc:474:replace_alu$46.lcu.$and$<techmap.v>:221$84_Y": {
          "hide_name": 1,
          "bits": [ 17 ],
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:221"
          }
        },
        "$techmap$techmap$add$sample.v:5$1.$auto$alumacc.cc:474:replace_alu$46.lcu.$and$<techmap.v>:221$87_Y": {
          "hide_name": 1,
          "bits": [ 20 ],
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:221"
          }
        },
        "$techmap$techmap$add$sample.v:5$1.$auto$alumacc.cc:474:replace_alu$46.lcu.$and$<techmap.v>:221$90_Y": {
          "hide_name": 1,
          "bits": [ 23 ],
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:221"
          }
        },
        "$techmap$techmap$add$sample.v:5$1.$auto$alumacc.cc:474:replace_alu$46.lcu.$and$<techmap.v>:222$89_Y": {
          "hide_name": 1,
          "bits": [ 21 ],
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:222"
          }
        },
        "$techmap$techmap$add$sample.v:5$1.$auto$alumacc.cc:474:replace_alu$46.lcu.$and$<techmap.v>:229$93_Y": {
          "hide_name": 1,
          "bits": [ 25 ],
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:229"
          }
        },
        "$techmap$techmap$add$sample.v:5$1.$auto$alumacc.cc:474:replace_alu$46.lcu.$or$<techmap.v>:221$88_Y": {
          "hide_name": 1,
          "bits": [ 28 ],
          "attributes": {
            "src": "sample.v:5|<techmap.v>:260|<techmap.v>:221"
          }
        },
        "A": {
          "hide_name": 0,
          "bits": [ 2, 3, 4, 5 ],
          "attributes": {
            "src": "sample.v:2"
          }
        },
        "B": {
          "hide_name": 0,
          "bits": [ 6, 7, 8, 9 ],
          "attributes": {
            "src": "sample.v:2"
          }
        },
        "X": {
          "hide_name": 0,
          "bits": [ 10, 11, 12, 13 ],
          "attributes": {
            "src": "sample.v:3"
          }
        },
        "carry": {
          "hide_name": 0,
          "bits": [ 14 ],
          "attributes": {
            "src": "sample.v:4"
          }
        }
      }
    }
  }
}
