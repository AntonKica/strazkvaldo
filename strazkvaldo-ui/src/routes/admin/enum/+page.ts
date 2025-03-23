import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	return {
		enums: {
			roomType: {
				name: "Typ miestnosti",
				values: [
					"bathroom",
					"bedroom",
					"livingRoom",
					"kitchen",
					"balcony",
					"workRoom",
					"garage",
					"cellar",
					"other"
				]
			},
			activityType: {
				name: "Typ aktivít",
				values: [
					"washing",
					"mopping",
					"cleaning",
					"vacuuming",
					"dusting",
					"garbageDisposal",
					"other",
				]
			}
		}
	};
};