/* eslint-disable */
/* tslint:disable */
// @ts-nocheck
/*
 * ---------------------------------------------------------------
 * ## THIS FILE WAS GENERATED VIA SWAGGER-TYPESCRIPT-API        ##
 * ##                                                           ##
 * ## AUTHOR: acacode                                           ##
 * ## SOURCE: https://github.com/acacode/swagger-typescript-api ##
 * ---------------------------------------------------------------
 */

export interface SalvoCoreHttpErrorsStatusErrorStatusError {
  brief: string;
  cause?: string;
  /**
   * @format uint16
   * @min 0
   */
  code: number;
  detail: string;
  name: string;
}

export type ServerEmpty = object;

export interface ServerRoutersAuthLoginData {
  account: string;
  password: string;
}

export interface ServerRoutersAuthLoginOutData {
  /** @format int64 */
  exp: number;
  id: string;
  token: string;
  username: string;
}
