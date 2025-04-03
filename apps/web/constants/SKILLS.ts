// Years of exp -> Skills list
const SKILLS_MAP: Map<number, string[]> = new Map([
   [0, []],
]);

export const SKILLS: {
   name: string
   years: number
}[] = Array.from(SKILLS_MAP).flatMap(([years, skills]) => skills.map((skill) => ({ name: skill, years })))

