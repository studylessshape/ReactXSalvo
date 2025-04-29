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

import {
  ServerCommonAppResponseServerRoutersAuthLoginOutData,
  ServerCommonAppResponseServerRoutersUserUserInfo,
  ServerCommonAppResponseServerRoutersUserUserOutData,
  ServerRoutersAuthLoginData,
  ServerRoutersUserAddUserInData,
} from "./data-contracts";
import { ContentType, HttpClient, RequestParams } from "./http-client";

export class Api<
  SecurityDataType = unknown,
> extends HttpClient<SecurityDataType> {
  /**
   * No description
   *
   * @tags Auth
   * @name ServerRoutersAuthPostLogin
   * @request POST:/api/auth/login
   */
  serverRoutersAuthPostLogin = (
    data: ServerRoutersAuthLoginData,
    params: RequestParams = {},
  ) =>
    this.request<ServerCommonAppResponseServerRoutersAuthLoginOutData, any>({
      path: `/api/auth/login`,
      method: "POST",
      body: data,
      type: ContentType.Json,
      format: "json",
      ...params,
    });
  /**
   * No description
   *
   * @tags User
   * @name ServerRoutersUserAddUser
   * @request POST:/api/user/add
   */
  serverRoutersUserAddUser = (
    data: ServerRoutersUserAddUserInData,
    params: RequestParams = {},
  ) =>
    this.request<ServerCommonAppResponseServerRoutersUserUserOutData, any>({
      path: `/api/user/add`,
      method: "POST",
      body: data,
      type: ContentType.Json,
      format: "json",
      ...params,
    });
  /**
   * No description
   *
   * @tags User
   * @name ServerRoutersUserGetUserProfile
   * @request GET:/api/user/profile
   */
  serverRoutersUserGetUserProfile = (params: RequestParams = {}) =>
    this.request<ServerCommonAppResponseServerRoutersUserUserInfo, any>({
      path: `/api/user/profile`,
      method: "GET",
      format: "json",
      ...params,
    });
}
