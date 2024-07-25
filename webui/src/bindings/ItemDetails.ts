// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ItemEvent } from "./ItemEvent";

export type ItemDetails = { 
/**
 * Id of the item
 */
id: bigint, 
/**
 * Name of the item
 */
name: string, 
/**
 * Optional inspection period
 */
inspection_period_days: number | null, 
/**
 * Optional serial number
 */
serial_number: string | null, 
/**
 * Ids of all tags associated to this item
 */
tags: Array<bigint>, 
/**
 * Events for this item
 */
events: Array<ItemEvent>, };