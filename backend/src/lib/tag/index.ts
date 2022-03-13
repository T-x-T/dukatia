import { Pool } from "pg";
import database from "./database.js";
import restApi from "./restApi.js";

type ITag = {
	id?: number,
	name: string,
	userId: number,
	parentId?: number
}

export type  { ITag };


const _tag = {
	async init(db: Pool) {
		database.init(db);
		restApi();
	},
	
	async getAll() {
		return (await database.getAll()).map(x => turnRowIntoITag(x));
	},
	
	async getById(id: number) {
		return turnRowIntoITag(await database.getById(id));
	},
	
	async add(tag: ITag): Promise<ITag> {
		if(typeof tag.parentId == "number" && !await isTagIdValidParent(tag.parentId)) throw new Error("specified parentTagId is invalid");
		const res = await database.add(tag);
		return turnRowIntoITag(res);
	},
	
	async update(tag: ITag) {
		if(!Number.isInteger(tag.id)) throw new Error("no valid id specified");
		if(typeof tag.parentId == "number" && !await isTagIdValidParent(tag.parentId, tag.id)) throw new Error("specified parentTagId is invalid");
		const res = await database.update(tag);
		if(res.length === 0) throw new Error("no row with id: " + tag.id);
		return turnRowIntoITag(res[0]);
	}
}

export default _tag;

function turnRowIntoITag(row: any): ITag {
	if(!row) return null;
	return {
		id: row.id,
		name: row.name,
		userId: row.user,
		parentId: typeof row.parent == "number" ? row.parent : undefined
	}
}

async function isTagIdValidParent(parentId: number, id?: number): Promise<boolean> {
	if(!await _tag.getById(parentId)) return false;

	if(typeof id != "number") return true;

	let nextParentToCheck = parentId;
	while(true) {
		if(nextParentToCheck === id) return false;
		const next = await _tag.getById(nextParentToCheck);
		if(!next) break;
		nextParentToCheck = next.parentId;
	}

	return true;
}