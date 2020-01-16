---
tags: [Configuration]
title: Karabiner Customized
created: '2020-01-15T14:19:15.323Z'
modified: '2020-01-15T14:21:06.537Z'
---

# Karabiner Customized

In `~/.config/karabiner/assets/complex_modifications/karabiner-programming.json`
```json
{
    "title": "Use grave accent key to simulate down and up arrow in specific programming applications",
    "rules": [
        {
            "description": "Press right option and grave accent key to simulate original grave accent key",
            "manipulators": [
                {
                    "type": "basic",
                    "from": {
                        "key_code": "grave_accent_and_tilde",
                        "modifiers": {
                            "mandatory": [
                                "right_option",
                                "left_shift"
                            ],
                            "optional": [
                                "any"
                            ]
                        }
                    },
                    "to": [
                        {
                            "key_code": "grave_accent_and_tilde",
                            "modifiers": "left_shift"
                        }
                    ],
                    "conditions": [
                        {
                            "type": "frontmost_application_if",
                            "bundle_identifiers": [
                                "^com\\.microsoft\\.VSCode$",
                                "^com\\.jetbrains\\.CLion$",
                                "^com\\.jetbrains\\.pycharm$",
                                "^org\\.godotengine\\.godot$",
                                "com\\.coteditor\\.CotEditor"
                            ]
                        }
                    ]
                },
                {
                    "type": "basic",
                    "from": {
                        "key_code": "grave_accent_and_tilde",
                        "modifiers": {
                            "mandatory": [
                                "right_option"
                            ],
                            "optional": [
                                "any"
                            ]
                        }
                    },
                    "to": [
                        {
                            "key_code": "grave_accent_and_tilde"
                        }
                    ],
                    "conditions": [
                        {
                            "type": "frontmost_application_if",
                            "bundle_identifiers": [
                                "^com\\.microsoft\\.VSCode$",
                                "^com\\.jetbrains\\.CLion$",
                                "^com\\.jetbrains\\.pycharm$",
                                "^org\\.godotengine\\.godot$",
                                "com\\.coteditor\\.CotEditor"
                            ]
                        }
                    ]
                }
            ]
        },
        {
            "description": "Press grave accent key to toggle down arrow",
            "manipulators": [
                {
                    "type": "basic",
                    "from": {
                        "key_code": "grave_accent_and_tilde",
                        "modifiers": {
                            "mandatory": [
                                "left_shift"  
                            ],
                            "optional": [
                                "any"
                            ]
                        }
                    },
                    "to": [
                        {
                            "key_code": "up_arrow"
                        }
                    ],
                    "conditions": [
                        {
                            "type": "frontmost_application_if",
                            "bundle_identifiers": [
                                "^com\\.microsoft\\.VSCode$",
                                "^com\\.jetbrains\\.CLion$",
                                "^com\\.jetbrains\\.pycharm$",
                                "^org\\.godotengine\\.godot$",
                                "com\\.coteditor\\.CotEditor"
                            ]
                        }
                    ]
                },
                {
                    "type": "basic",
                    "from": {
                        "key_code": "grave_accent_and_tilde",
                        "modifiers": {
                            "optional": [
                                "any"
                            ]
                        }
                    },
                    "to": [
                        {
                            "key_code": "down_arrow"
                        }
                    ],
                    "conditions": [
                        {
                            "type": "frontmost_application_if",
                            "bundle_identifiers": [
                                "^com\\.microsoft\\.VSCode$",
                                "^com\\.jetbrains\\.CLion$",
                                "^com\\.jetbrains\\.pycharm$",
                                "^org\\.godotengine\\.godot$",
                                "com\\.coteditor\\.CotEditor"
                            ]
                        }
                    ]
                }
            ]
        }
    ]
}
```
