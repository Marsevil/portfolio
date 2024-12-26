export type PersonalInfos = {
	name: string;
	title: string;
	description: string;
};

export type LanguageEntry = {
	name: string;
	level: number;
	comment: string | undefined;
};

export type EducationEntry = {
	title: string;
	location: string;
	institution: string;
	comment: string | undefined;
	period: [Date, Date];
};

export type ProfessionalEntry = {
	title: string;
	company: string;
	location: string;
	period: [Date, Date];
	description: string;
	tasks: string[];
	technos: string[];
};

export type ProjectState = 'wip' | 'done' | 'aborted';

export type ProjectEntry = {
	title: string;
	description: string;
	state: ProjectState;
	technos: string[];
	link: string | undefined;
	sourceLink: string | undefined;
};

export type Content = {
	personal: PersonalInfos;
	devLanguages: LanguageEntry[];
	humanLanguages: LanguageEntry[];
	educations: EducationEntry[];
	experiences: ProfessionalEntry[];
	projects: ProjectEntry[];
};
