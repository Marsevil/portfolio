export type Titles = {
	description: string;
	language: string;
	humanLanguage: string;
	devLanguage: string;
	project: string;
	road: string;
};

export type Banner = {
	interactDialog: string;
};

export type ProjectCard = {
	sourceLink: string;
	stateTag: {
		done: string;
		wip: string;
		aborted: string;
	};
};

export type DialogBox = {
	close: string;
};

export type Vars = {
	rightReserved: string;
	titles: Titles;
	projectCard: ProjectCard;
	dialogBox: DialogBox;
	banner: Banner;
};
