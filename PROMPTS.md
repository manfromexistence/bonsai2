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

Create a ai agent orb that will its eyes and one mouth animated the eye will blink and will have all kinds of hover, click, dblclick and all kinds of interactions response in cute face and will have physics behind it and will be complete by code and new emotion creator correctly so we have to put serious smooth framer motion animations in it and will be like these images - and mostly be vercel dark theme by default with vercel light theme and on strong emotions there will rainbow aniamtion smooth color glow around its circle correctly!!!

This is a system instruction prompt dx-tui and its current 495 tokens so please first rebrand this as "Dx - Enhanced Development Experience" Its a superapp created by EssenceFromExistence and dx has a entire ecosystem in it with tui, desktop, mobile, cli, website and its the fastest, cheapest, most performant one

1. tool schema = 950
2. dx serializer compact/tool schema helper = 25
3. system prompt = 125
4. user info = 25
5. git status = 25
6. mcp+skills+hooks+plugins+connects+channels = 50
7. dx tools = 50

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

1. First Prompt + Dx tools
2. Oauth: Codex, Claude, Cursor
3. Channels
4. Plugins
5. Connects

So after taking actions via openclaw, and hermes-agent's auto update skills loop we now have grok bot's record a skills so what is the latest ai agents powers that can go viral as I am creating my ai agent so I need ideas that are thses types of uniqe and game changing!!!

I have a game changing latest and unique idea - so latest grok build has a record a screen but we will have all the time daemon that will record whatever you do in the screens like  a log in markdown files like it will have like this:
1:08PM=actioins
So its record a screen but in text markdown that any agents can then just search that markdown file with the time frame so anyone can just tell ai agents that what I did with an app or any time and the ai agents check when the user did and user did like the keyboard inputs, mouse postions and app opened in what positions or state


Its the best so its records all the things the user did in markdown text so any agents can use this and keeps user privacy at the same time as its just records actions in texts so its becomes just enough good state for usefull but not that it drains too much resources as its will just records user actions and if some times actiosn are same then there will be same text and we will divide this in files and folders so its not a gian file that we can't feed to ai agent - so it sucsessfully records everything and in the best way possible and openclaw was 24/7 agent buts it not only does everything but also does 24/7 recording - now we can do that cause we are in rust so many less softwares has this so openclaw, hermes-agent or others can't do it the best way as rust is the best language that can pull this of - keeping alive all the time but still be nothing a big fat deaemon!!!

So we can be more viral than openclaw - just think about it!!!

That is awesome now in our model picker we have hardcoded the local 2 models so please remove that there won't be like that hardcoded models so in the default free models like the grok models and the opencode zen free models + openroute or anyt other free models without any api keys and then connected providers models and make sure alongisde the models we will show `[model name] (provider name)` in the /models suggestions and is there any way to shows opensource gglm models and make search filter so please in /model suggestions box please we will gglm model names and clicking on them we will download those model in the os dx cache folder like LOCALDATA dx/flow/models/llm folder and we will show the models in that folder on any os in our tui in the /models suggestion not hardcoded models name!!!

That is awesome - now the whole point of our dx-memory is then when the dx-memory daemon is on the markdown text will see in a way so that by reading those logs any ai agent can know what the user is doing so please start that daemon and tell me what I am doing right now and if it matches my actual task them our dx-memory is complete and also tell me what is the files and folders are we using  for our dx-memory markdown file creation and how much os ram and other resources are we using and if its not hardware best os performance then plase keep making our dx-memory the best possible to be in hardware limit as its need to be working all the time on user os so it needs to be battle-tested, production-ready and professional 100 out of 100!!!

Is there any groundbreaking packages turn images to the most efficent sized data so it can take time to process but I want to be moved liked the smallest version of that image - as you are right that we should make our dx-memory more better as our text will be the default one but we will also support miages on big events that will make our dx-memory the best!!! so please best images compressions or like zstd to xomplesss it and how much we can comporess it??

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

1. Other interactions of up and down arrow should be silent while the extension menu is open
2. Auto Update
3. Agent
4. Record Actions
5. Channels
6. Plugins
7. Connects

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


Now, don't do anything and just understand - we use xai grok build fork and created our dx-tui as adding lots of features so yeah we have keep up to date with as they are a big company so free work from them so why not by our top priority that our dx-tui based features all stays as in the grok-tui from them they have topbar in a tui, bro its a waste and we moved it to our bottom bar right and then the chatinputbox top line at bottombar left saving space for the actual messages what actually matter so they update they do there should reflect on our tui bottombar right so do you understand the situation and create AGENT.md file so every next ai agents understand this!!!

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

That is awesome now please create first AGENTS.md file with TODO.md, CHANGELOG.md file and update README.md file with what you are doing and out of 100% what is the status so everything you do falls correctlys and professional 100 out of 100, production-ready!!!

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


In the tool scheme we are sending 26 tools right?? use "token help" and its ai command to know about the token

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Now, in our dx-desktop we show statusbar center icons to run website in the g:dx/desktop/web folder that we build and then copy to g:dx/desktop/assets/web folder right?? so previously video and the 3d was not working so first rebuild all the web project and copy them to there and first fix the grapics folder as its resize is by default 250 but please make it 100 by default + safely remove design and presentation project + show router project and also the router icon is not showing in the statusbar so please fix that too + at the shader(there is loading button to load the shader - please make sure that it auto laods correctly, whiteboard and metasearch(currnetly not running - we need to run the dx-metasearch and run it correctly while running) are dx-www project - whiteboard and shader already works but metasearch is not working + and at the train project its needs llama-sever correctly running with other stuffs so please fix that correctly - you can learn about it from g:dx/www folder about dx-www and to know more about metasearch please look at the g:dx/metasearch folder

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Please look at the flow project as there is so much pages with so much dummy data so like only keep the pages that are we are mostly and commentout other pages don't delete just comment or just include in the sidebar for now and like organize that we are mostly using correctly + and then run the project and look at the g:Dx/i18n project and check does its tts and stt with google translate works and if so then please include that in our flow by copying that project to the flow project and alonside the local stt and tts also put of remote not mention google translate just mention remote stt and tts correctly

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Please create a manga that starts as a gojo like character very sick and its best geto like character says in your whole life you didn't got sick for once until now at 21 years old just where in your mind you inserted a man called "Mahatab Hossain Sumon(EssenceFromExistence)" in you - Even through you are strongest in this era and even weak people after brain connect gets stronger but you are getting sick why - then their teacher says it can in many cases mostly because the man's brain came from a complete remote place from the universe it was so far that we can even say from where its brain connect worked - most cases the brain connect it got may be a very weak in health or something else we didn't seen before

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Please create a latest marketplace commerce project with latest bun+tailwindcss+shadcnui+nextjs stack with backend and frontend, and latest theme based... and use latest nextjs server components for the server. So we will do both frontend and backend on the same project. And it will be a Shopify like project where we will create store fronts to the user. And it will be like an Arabian Aladdin golden shiny golden theme based project. And fully responsive on mobile devices. On we will use Framer Motion for animation. And this project will have many kind of interaction. In the header on mobile screen, on the left side we will have a two line hamburger icon. Then with then a left site we will have a notification icon. And on the center there will be our golden shiny Aladdin logo. And on the right we will have a search bar and a avatar icon. In here we will use latest shadcn-ui components library and use latest shadcn-ui packages too, correctly. So like it will have a lot of features and lot of gesture. Like we can scroll to the top bar, push down the top bar or push up the bottom bar. And like we can have bottom bar on mobile. On the left we will have home icon, then subscription icon, then add icon and then cart and library icon. So it will have both buyer and seller mode. This is not a only one type of mobile app, e-commerce app. Like it will have all the features of Shopify, Amazon, Airbnb, freelancer platform like Fiverr, Upwork, freelancer. The goal is to like make the commerce great both online and offline not gready money taking platforms so anyone can  can buy and sell anything both online and offline or both in the same time free unlimited. And here is the requirements.

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Please create a latest nextjs+bun+tailwindcss+shadcnui+framer-motion stack project with only a cursome that moves with animations and when moving it shows cursor flash and also it shows rainbow animation on the cursor and also has a bottom status with a editor like line details and multiple user avater on the statusbar right side correctly only that cursor and code editor like bottom statusbar nothing else!!!

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Many people's are using ai to make latest breakthrough in mathematics so I wanna do that too - so I have a grok 4.6 model so please tell me what are the current mathematics problems easiest still unsolved question?

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Now in here don't change anything in the tools as we are sending dx serializer compact format for ai correctly to save tokens on the first prompt and for most the tools its working correctly - now just create a new tool to call other tools full schema - now you will be confused that how we are telling about the tui tools with sending the 15k tools json - the answer is our dx-serializer compat and for that we are just sending around 1-2k on the first prompt now so don't please change it just create another tool called get tool details or something like that so the ai can use it know about the tools schema that the don't understand and only use that if the fail to open run that tools after trying 3rd time and create or update the local AGENTS.md file with the tools schema but not the whole tools just what the ai wrong in the AGENHTS.md file correctly

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------



----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Web Tools
1. Video
2. Cms
3. Graph
4. Train
5. Metasearch
6. Router
7. Shader

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Please look at the g:dx/desktop/web/shader project that is a dx-www framework project - you can learn about it at the g:dx/www folder and in that project when we do dx dev on that project at the loader there is just loader but when we do dx build it output a project where there is a loader but also has a rectanglge behind it - and also our current dx whole system has 2 flaws - 1. so it don't knows what is the root of the project as its createing .dx folder everywhere and then 2. it create index.html, style.css and other one file when we use dx but we don't want that we may create our dx config file that is "dx" without any extension file in dx serializer human format but not any others so please tell me how to solves these problems?? and for the serializer you can look at the g:dx/serializer folder and the whole g:dx is filled with dx tools

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

now at the inspirations folder there is bloub folder - right so its in vue stack so please copy it and make that to our nextjs stack correctly - and create a new page called bot and in there recreate it in nextjs at our g:dx/mobile project ocrrectly

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

