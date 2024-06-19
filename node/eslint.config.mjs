// @ts-check

import eslint from '@eslint/js';
import globals from "globals";
import tseslint from 'typescript-eslint';

export default tseslint.config(
	eslint.configs.recommended,
	...tseslint.configs.recommended,
	{
		languageOptions: {
			globals: {
				...globals.node,
				...globals.jest,
			}
		},
		rules: {
			"@typescript-eslint/no-namespace": "off",
			"@typescript-eslint/no-unused-vars": [
				"error",
				{
					"argsIgnorePattern": "^_",
					"varsIgnorePattern": "^_",
					"destructuredArrayIgnorePattern": "^_",
					"caughtErrorsIgnorePattern": "^_",
				}
			]
		}
	}
);