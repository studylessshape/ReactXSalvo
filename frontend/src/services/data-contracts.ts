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

export interface ServerCommonAppResponseServerRoutersAuthLoginOutData {
  /**
   * @format uint16
   * @min 0
   */
  code: number;
  data?: null & ServerRoutersAuthLoginOutData;
  message?: string | null;
}

export interface ServerCommonAppResponseServerRoutersUserUserInfo {
  /**
   * @format uint16
   * @min 0
   */
  code: number;
  data?: null & ServerRoutersUserUserInfo;
  message?: string | null;
}

export interface ServerCommonAppResponseServerRoutersUserUserOutData {
  /**
   * @format uint16
   * @min 0
   */
  code: number;
  data?: null & ServerRoutersUserUserOutData;
  message?: string | null;
}

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

export interface ServerRoutersUserAddUserInData {
  account: string;
  password: string;
}

export interface ServerRoutersUserUserInfo {
  id: string;
  username: string;
}

export interface ServerRoutersUserUserOutData {
  id: string;
}
