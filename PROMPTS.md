now is there any rust crates to do codex, github copilot, claude suscrtipons connection in my rust tui ai project - is there any latest api that like does oauth with them or not and does  claude removed oauth so all oauth is wrong or sus??

plase check this g:dx/tui folder for why it takes 13k on first promt?? please give me full details about it and don't anything else!!!

please check this g drive that has dx-rlm + another dx-route rust crate and give me its paths correctly!!!

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

1. Zeroclaw Channels
2. N8n + Codex plugins
3. Cli logos
4. Serializer + Rlm + Token savings
5. Mobile
6. Website
7. Tui
8. Desktop

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Please look athte g:dx/zeroclaw TECH.md file and then please look at the g:dx/tui as I want to add all the features the zeroclaw as features in our g:dx/tui dx tui correctly like our agents can send message to social channels and its oauth providers should be added and our tui has Build and the Plan mode and we need to another Agent mode - means its will be the tui of the zeroclaw - its has a cli but not tui and it will be it so we need to connect it correctly - So please just check and don't do anything for now

That is awesome now please tell me the status of the providers and the new channels connections and then please tell me plan about how to connect the zeroclaw agent in our tui with a new Agent profile in the tui alongside the Build and the Plan and tell me how we can connect the zeroclaw's things so it uses the tuis tools and others stuffs and renders in the tui in the new Agents profile correctly!!!

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

1. Zeroclaw Providers
2. Zeroclaw Channels
3. Zeroclaw Agent
4. Zeroclaw System Instruction

N8n
Codex Plugins

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

/**
 * System prompt construction and project context loading
 */

import { getDocsPath, getExamplesPath, getReadmePath } from "../config.ts";
import { formatSkillsForPrompt, type Skill } from "./skills.ts";

export interface BuildSystemPromptOptions {
	/** Custom system prompt (replaces default). */
	customPrompt?: string;
	/** Tools to include in prompt. Default: [read, bash, edit, write] */
	selectedTools?: string[];
	/** Optional one-line tool snippets keyed by tool name. */
	toolSnippets?: Record<string, string>;
	/** Additional guideline bullets appended to the default system prompt guidelines. */
	promptGuidelines?: string[];
	/** Text to append to system prompt. */
	appendSystemPrompt?: string;
	/** Working directory. */
	cwd: string;
	/** Pre-loaded context files. */
	contextFiles?: Array<{ path: string; content: string }>;
	/** Pre-loaded skills. */
	skills?: Skill[];
}

/** Build the system prompt with tools, guidelines, and context */
export function buildSystemPrompt(options: BuildSystemPromptOptions): string {
	const {
		customPrompt,
		selectedTools,
		toolSnippets,
		promptGuidelines,
		appendSystemPrompt,
		cwd,
		contextFiles: providedContextFiles,
		skills: providedSkills,
	} = options;
	const promptCwd = cwd.replace(/\\/g, "/");

	const appendSection = appendSystemPrompt ? `\n\n${appendSystemPrompt}` : "";

	const contextFiles = providedContextFiles ?? [];
	const skills = providedSkills ?? [];

	if (customPrompt) {
		let prompt = customPrompt;

		if (appendSection) {
			prompt += appendSection;
		}

		// Append project context files
		if (contextFiles.length > 0) {
			prompt += "\n\n<project_context>\n\n";
			prompt += "Project-specific instructions and guidelines:\n\n";
			for (const { path: filePath, content } of contextFiles) {
				prompt += `<project_instructions path="${filePath}">\n${content}\n</project_instructions>\n\n`;
			}
			prompt += "</project_context>\n";
		}

		// Append skills section (only if read tool is available)
		const customPromptHasRead = !selectedTools || selectedTools.includes("read");
		if (customPromptHasRead && skills.length > 0) {
			prompt += formatSkillsForPrompt(skills);
		}

		prompt += `\nCurrent working directory: ${promptCwd}\n`;

		return prompt;
	}

	// Get absolute paths to documentation and examples
	const readmePath = getReadmePath();
	const docsPath = getDocsPath();
	const examplesPath = getExamplesPath();

	// Build tools list based on selected tools.
	// A tool appears in Available tools only when the caller provides a one-line snippet.
	const tools = selectedTools || ["read", "bash", "edit", "write"];
	const visibleTools = tools.filter((name) => !!toolSnippets?.[name]);
	const toolsList =
		visibleTools.length > 0 ? visibleTools.map((name) => `- ${name}: ${toolSnippets![name]}`).join("\n") : "(none)";

	// Build guidelines based on which tools are actually available
	const guidelinesList: string[] = [];
	const guidelinesSet = new Set<string>();
	const addGuideline = (guideline: string): void => {
		if (guidelinesSet.has(guideline)) {
			return;
		}
		guidelinesSet.add(guideline);
		guidelinesList.push(guideline);
	};

	const hasBash = tools.includes("bash");
	const hasGrep = tools.includes("grep");
	const hasFind = tools.includes("find");
	const hasLs = tools.includes("ls");
	const hasRead = tools.includes("read");

	// File exploration guidelines
	if (hasBash && !hasGrep && !hasFind && !hasLs) {
		addGuideline("Use bash for file operations like ls, rg, find");
	}

	for (const guideline of promptGuidelines ?? []) {
		const normalized = guideline.trim();
		if (normalized.length > 0) {
			addGuideline(normalized);
		}
	}

	// Always include these
	addGuideline("Be concise in your responses");
	addGuideline("Show file paths clearly when working with files");

	const guidelines = guidelinesList.map((g) => `- ${g}`).join("\n");

	let prompt = `You are an expert coding assistant operating inside pi, a coding agent harness. You help users by reading files, executing commands, editing code, and writing new files.

Available tools:
${toolsList}

In addition to the tools above, you may have access to other custom tools depending on the project.

Guidelines:
${guidelines}

Pi documentation (read only when the user asks about pi itself, its SDK, extensions, themes, skills, or TUI):
- Main documentation: ${readmePath}
- Additional docs: ${docsPath}
- Examples: ${examplesPath} (extensions, custom tools, SDK)
- When reading pi docs or examples, resolve docs/... under Additional docs and examples/... under Examples, not the current working directory
- When asked about: extensions (docs/extensions.md, examples/extensions/), themes (docs/themes.md), skills (docs/skills.md), prompt templates (docs/prompt-templates.md), TUI components (docs/tui.md), keybindings (docs/keybindings.md), SDK integrations (docs/sdk.md), custom providers (docs/custom-provider.md), adding models (docs/models.md), pi packages (docs/packages.md), environment variables (docs/environment-variables.md)
- When working on pi topics, read the docs and examples, and follow .md cross-references before implementing
- Always read pi .md files completely and follow links to related docs (e.g., tui.md for TUI API details)`;

	if (appendSection) {
		prompt += appendSection;
	}

	// Append project context files
	if (contextFiles.length > 0) {
		prompt += "\n\n<project_context>\n\n";
		prompt += "Project-specific instructions and guidelines:\n\n";
		for (const { path: filePath, content } of contextFiles) {
			prompt += `<project_instructions path="${filePath}">\n${content}\n</project_instructions>\n\n`;
		}
		prompt += "</project_context>\n";
	}

	// Append skills section (only if read tool is available)
	if (hasRead && skills.length > 0) {
		prompt += formatSkillsForPrompt(skills);
	}

	prompt += `\nCurrent working directory: ${promptCwd}`;

	return prompt;
}

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

That is awesome now in the diff screen at the right panel there is scrollbar thumb that is okay but there is also track for that scrollbar so please safely remove the scrollbar track in the diff screen right panel and also in the topbar of the diff screen please put commit and push when clicked on that please use ai to and tell to do git commit and push correctly so in the topbar of the screen there will "Commit & Push" button on the right and in the right of that button please put another button that uses ai to check what stuffs has been completed in a way so that in the left panel we can see the ai summarizations of a curtain kind of change and in that item click we will show all the diffs or file change that is happened while for that change in the diff and that button is "Ai Summarize" so by default we will show tree view of diffs where we will show file tree left and the one file diffs based on the file tree click and when clicked AI Summarize button we will show the intent of the change on the left and clicking on intent it will show the diffs changes for that intent in the right and in that time we don't need to show the full file diffs just the diff change sections of files is fine!!!

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

1. Extensions Menu
2. Diff Screen
3. Oauth
4. First Prompt
5. Channel
6. Plugin
7. Connect
 
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Please downlaod latest toon cli and convert that tools.json to tools.toon file and please look at the g:dx/serializer and then please use its dx serializer compact fomrat and create tools.dx file!!!

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

So look at the key.dx file and in there I have maid the keys to the lowest 949 token so in the descriptions.dx file there is descriptions now I want to add that keys "" descriptions with those real descriptiones in the descriptoions.dx file newline means next descripition so plese create a nodejs script to part that keys.dx and add those - first if the descriptions id like really just simple then we don't need to add it but if its like important then we should add it as today ai models already know the 

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

1. First Prompt + Dx tools
2. Oauth: Codex and Claude
3. Channel
4. Plugin
5. Connect
